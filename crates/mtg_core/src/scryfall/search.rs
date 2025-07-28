use super::client::ScryfallClient;
use super::types::*;
use color_eyre::Result;
use std::collections::HashMap;

impl ScryfallClient {
    /// Search for cards using a query string
    pub async fn search_cards(&self, params: SearchParams) -> Result<SearchResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("q".to_string(), params.q);

        if let Some(unique) = params.unique {
            query_params.insert("unique".to_string(), unique);
        }
        if let Some(order) = params.order {
            query_params.insert("order".to_string(), order);
        }
        if let Some(dir) = params.dir {
            query_params.insert("dir".to_string(), dir);
        }
        if let Some(include_extras) = params.include_extras {
            query_params.insert("include_extras".to_string(), include_extras.to_string());
        }
        if let Some(include_multilingual) = params.include_multilingual {
            query_params.insert(
                "include_multilingual".to_string(),
                include_multilingual.to_string(),
            );
        }
        if let Some(include_variations) = params.include_variations {
            query_params.insert(
                "include_variations".to_string(),
                include_variations.to_string(),
            );
        }
        if let Some(page) = params.page {
            query_params.insert("page".to_string(), page.to_string());
        }

        self.get_with_params("cards/search", query_params).await
    }

    /// Get a card by exact name
    pub async fn get_card_named(&self, name: &str, set: Option<&str>) -> Result<Card> {
        let mut query_params = HashMap::new();
        query_params.insert("exact".to_string(), name.to_string());

        if let Some(set_code) = set {
            query_params.insert("set".to_string(), set_code.to_string());
        }

        self.get_with_params("cards/named", query_params).await
    }

    /// Get a card by fuzzy name matching
    pub async fn get_card_fuzzy(&self, name: &str) -> Result<Card> {
        let mut query_params = HashMap::new();
        query_params.insert("fuzzy".to_string(), name.to_string());

        self.get_with_params("cards/named", query_params).await
    }

    /// Get a card by Scryfall ID
    pub async fn get_card_by_id(&self, id: &str) -> Result<Card> {
        self.get(&format!("cards/{id}")).await
    }

    /// Get a card by set code and collector number
    pub async fn get_card_by_collector(
        &self,
        set_code: &str,
        collector_number: &str,
        lang: Option<&str>,
    ) -> Result<Card> {
        let mut endpoint = format!("cards/{set_code}/{collector_number}");

        if let Some(language) = lang {
            endpoint.push_str(&format!("/{language}"));
        }

        self.get(&endpoint).await
    }

    /// Get a card by Arena ID
    pub async fn get_card_by_arena_id(&self, arena_id: u32) -> Result<Card> {
        self.get(&format!("cards/arena/{arena_id}")).await
    }

    /// Get a card by MTGO ID
    pub async fn get_card_by_mtgo_id(&self, mtgo_id: u32) -> Result<Card> {
        self.get(&format!("cards/mtgo/{mtgo_id}")).await
    }

    /// Get a card by Multiverse ID
    pub async fn get_card_by_multiverse_id(&self, multiverse_id: u32) -> Result<Card> {
        self.get(&format!("cards/multiverse/{multiverse_id}")).await
    }

    /// Get a card by TCGPlayer ID
    pub async fn get_card_by_tcgplayer_id(&self, tcgplayer_id: u32) -> Result<Card> {
        self.get(&format!("cards/tcgplayer/{tcgplayer_id}")).await
    }

    /// Get a card by Cardmarket ID
    pub async fn get_card_by_cardmarket_id(&self, cardmarket_id: u32) -> Result<Card> {
        self.get(&format!("cards/cardmarket/{cardmarket_id}")).await
    }

    /// Get a random card
    pub async fn get_random_card(&self, query: Option<&str>) -> Result<Card> {
        if let Some(q) = query {
            let mut query_params = HashMap::new();
            query_params.insert("q".to_string(), q.to_string());
            self.get_with_params("cards/random", query_params).await
        } else {
            self.get("cards/random").await
        }
    }

    /// Get autocomplete suggestions
    pub async fn autocomplete(
        &self,
        query: &str,
        include_extras: Option<bool>,
    ) -> Result<AutocompleteResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("q".to_string(), query.to_string());

        if let Some(extras) = include_extras {
            query_params.insert("include_extras".to_string(), extras.to_string());
        }

        self.get_with_params("cards/autocomplete", query_params)
            .await
    }

    /// Build an advanced search query from parameters
    pub fn build_advanced_query(&self, params: &AdvancedSearchParams) -> String {
        let mut query_parts = Vec::new();

        if let Some(name) = &params.name {
            if name.contains(' ') {
                query_parts.push(format!("\"{name}\""));
            } else {
                query_parts.push(name.clone());
            }
        }

        if let Some(oracle) = &params.oracle {
            query_parts.push(format!("o:{oracle}"));
        }

        if let Some(card_type) = &params.card_type {
            query_parts.push(format!("t:{card_type}"));
        }

        if let Some(colors) = &params.colors {
            query_parts.push(self.format_color_query(colors));
        }

        if let Some(identity) = &params.identity {
            query_parts.push(self.format_color_identity_query(identity));
        }

        if let Some(mana) = &params.mana {
            query_parts.push(format!("m:{mana}"));
        }

        if let Some(mv) = &params.mv {
            query_parts.push(format!("mv{}", self.format_comparison(mv)));
        }

        if let Some(power) = &params.power {
            query_parts.push(format!("pow{}", self.format_comparison(power)));
        }

        if let Some(toughness) = &params.toughness {
            query_parts.push(format!("tou{}", self.format_comparison(toughness)));
        }

        if let Some(loyalty) = &params.loyalty {
            query_parts.push(format!("loy{}", self.format_comparison(loyalty)));
        }

        if let Some(set) = &params.set {
            query_parts.push(format!("s:{set}"));
        }

        if let Some(rarity) = &params.rarity {
            query_parts.push(format!("r:{rarity}"));
        }

        if let Some(artist) = &params.artist {
            query_parts.push(format!("a:{artist}"));
        }

        if let Some(flavor) = &params.flavor {
            query_parts.push(format!("ft:{flavor}"));
        }

        if let Some(format) = &params.format {
            query_parts.push(format!("f:{format}"));
        }

        if let Some(language) = &params.language {
            query_parts.push(format!("lang:{language}"));
        }

        query_parts.join(" ")
    }

    /// Search using advanced parameters
    pub async fn search_advanced(&self, params: AdvancedSearchParams) -> Result<SearchResponse> {
        let query = self.build_advanced_query(&params);

        let search_params = SearchParams {
            q: query,
            unique: params.unique,
            order: params.order,
            dir: params.dir,
            include_extras: params.include_extras,
            include_multilingual: params.include_multilingual,
            include_variations: params.include_variations,
            page: params.page,
        };

        self.search_cards(search_params).await
    }

    /// Search for creatures with filters
    pub async fn search_creatures(
        &self,
        color: Option<String>,
        power: Option<String>,
        toughness: Option<String>,
        mana_value: Option<String>,
        format: Option<String>,
    ) -> Result<SearchResponse> {
        let mut query_parts = vec!["t:creature".to_string()];

        if let Some(c) = color {
            query_parts.push(self.format_color_query(&c));
        }

        if let Some(p) = power {
            query_parts.push(format!("pow{}", self.format_comparison(&p)));
        }

        if let Some(t) = toughness {
            query_parts.push(format!("tou{}", self.format_comparison(&t)));
        }

        if let Some(mv) = mana_value {
            query_parts.push(format!("mv{}", self.format_comparison(&mv)));
        }

        if let Some(f) = format {
            query_parts.push(format!("f:{f}"));
        }

        let search_params = SearchParams {
            q: query_parts.join(" "),
            ..Default::default()
        };

        self.search_cards(search_params).await
    }

    /// Search for instants with filters
    pub async fn search_instants(
        &self,
        color: Option<String>,
        mana_value: Option<String>,
        format: Option<String>,
    ) -> Result<SearchResponse> {
        let mut query_parts = vec!["t:instant".to_string()];

        if let Some(c) = color {
            query_parts.push(self.format_color_query(&c));
        }

        if let Some(mv) = mana_value {
            query_parts.push(format!("mv{}", self.format_comparison(&mv)));
        }

        if let Some(f) = format {
            query_parts.push(format!("f:{f}"));
        }

        let search_params = SearchParams {
            q: query_parts.join(" "),
            ..Default::default()
        };

        self.search_cards(search_params).await
    }

    /// Search for sorceries with filters
    pub async fn search_sorceries(
        &self,
        color: Option<String>,
        mana_value: Option<String>,
        format: Option<String>,
    ) -> Result<SearchResponse> {
        let mut query_parts = vec!["t:sorcery".to_string()];

        if let Some(c) = color {
            query_parts.push(self.format_color_query(&c));
        }

        if let Some(mv) = mana_value {
            query_parts.push(format!("mv{}", self.format_comparison(&mv)));
        }

        if let Some(f) = format {
            query_parts.push(format!("f:{f}"));
        }

        let search_params = SearchParams {
            q: query_parts.join(" "),
            ..Default::default()
        };

        self.search_cards(search_params).await
    }

    /// Search for planeswalkers with filters
    pub async fn search_planeswalkers(
        &self,
        color: Option<String>,
        loyalty: Option<String>,
        format: Option<String>,
    ) -> Result<SearchResponse> {
        let mut query_parts = vec!["t:planeswalker".to_string()];

        if let Some(c) = color {
            query_parts.push(self.format_color_query(&c));
        }

        if let Some(l) = loyalty {
            query_parts.push(format!("loy{}", self.format_comparison(&l)));
        }

        if let Some(f) = format {
            query_parts.push(format!("f:{f}"));
        }

        let search_params = SearchParams {
            q: query_parts.join(" "),
            ..Default::default()
        };

        self.search_cards(search_params).await
    }

    /// Search for commanders (legendary creatures)
    pub async fn search_commanders(
        &self,
        identity: Option<String>,
        mana_value: Option<String>,
    ) -> Result<SearchResponse> {
        let mut query_parts = vec!["t:legendary t:creature".to_string()];

        if let Some(id) = identity {
            query_parts.push(self.format_color_identity_query(&id));
        }

        if let Some(mv) = mana_value {
            query_parts.push(format!("mv{}", self.format_comparison(&mv)));
        }

        let search_params = SearchParams {
            q: query_parts.join(" "),
            ..Default::default()
        };

        self.search_cards(search_params).await
    }

    // Helper methods for query formatting
    fn format_color_query(&self, color: &str) -> String {
        let color = color.to_lowercase();

        match color.as_str() {
            "white" => "c:w".to_string(),
            "blue" => "c:u".to_string(),
            "black" => "c:b".to_string(),
            "red" => "c:r".to_string(),
            "green" => "c:g".to_string(),
            "colorless" => "c:colorless".to_string(),
            "multicolor" => "c:m".to_string(),
            // Guild names
            "azorius" => "c:wu".to_string(),
            "dimir" => "c:ub".to_string(),
            "rakdos" => "c:br".to_string(),
            "gruul" => "c:rg".to_string(),
            "selesnya" => "c:gw".to_string(),
            "orzhov" => "c:wb".to_string(),
            "izzet" => "c:ur".to_string(),
            "golgari" => "c:bg".to_string(),
            "boros" => "c:rw".to_string(),
            "simic" => "c:gu".to_string(),
            // Shard names
            "bant" => "c:gwu".to_string(),
            "esper" => "c:wub".to_string(),
            "grixis" => "c:ubr".to_string(),
            "jund" => "c:brg".to_string(),
            "naya" => "c:rgw".to_string(),
            // Wedge names
            "abzan" => "c:wbg".to_string(),
            "jeskai" => "c:urw".to_string(),
            "sultai" => "c:bgu".to_string(),
            "mardu" => "c:rwb".to_string(),
            "temur" => "c:gur".to_string(),
            _ => {
                if color.starts_with("c:") {
                    color
                } else {
                    format!("c:{color}")
                }
            }
        }
    }

    fn format_color_identity_query(&self, identity: &str) -> String {
        let identity = identity.to_lowercase();

        match identity.as_str() {
            "white" => "id:w".to_string(),
            "blue" => "id:u".to_string(),
            "black" => "id:b".to_string(),
            "red" => "id:r".to_string(),
            "green" => "id:g".to_string(),
            "colorless" => "id:colorless".to_string(),
            // Guild names
            "azorius" => "id:wu".to_string(),
            "dimir" => "id:ub".to_string(),
            "rakdos" => "id:br".to_string(),
            "gruul" => "id:rg".to_string(),
            "selesnya" => "id:gw".to_string(),
            "orzhov" => "id:wb".to_string(),
            "izzet" => "id:ur".to_string(),
            "golgari" => "id:bg".to_string(),
            "boros" => "id:rw".to_string(),
            "simic" => "id:gu".to_string(),
            // Shard names
            "bant" => "id:gwu".to_string(),
            "esper" => "id:wub".to_string(),
            "grixis" => "id:ubr".to_string(),
            "jund" => "id:brg".to_string(),
            "naya" => "id:rgw".to_string(),
            // Wedge names
            "abzan" => "id:wbg".to_string(),
            "jeskai" => "id:urw".to_string(),
            "sultai" => "id:bgu".to_string(),
            "mardu" => "id:rwb".to_string(),
            "temur" => "id:gur".to_string(),
            _ => {
                if identity.starts_with("id:") {
                    identity
                } else {
                    format!("id:{identity}")
                }
            }
        }
    }

    fn format_comparison(&self, value: &str) -> String {
        if value.starts_with(">=")
            || value.starts_with("<=")
            || value.starts_with("!=")
            || value.starts_with('>')
            || value.starts_with('<')
        {
            value.to_string()
        } else {
            format!("={value}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_advanced_query() {
        let client = ScryfallClient::new().expect("Failed to create client");

        let params = AdvancedSearchParams {
            name: Some("Lightning Bolt".to_string()),
            card_type: Some("instant".to_string()),
            colors: Some("red".to_string()),
            ..Default::default()
        };

        let query = client.build_advanced_query(&params);
        assert!(query.contains("\"Lightning Bolt\""));
        assert!(query.contains("t:instant"));
        assert!(query.contains("c:r"));
    }

    #[test]
    fn test_format_color_query() {
        let client = ScryfallClient::new().expect("Failed to create client");

        assert_eq!(client.format_color_query("red"), "c:r");
        assert_eq!(client.format_color_query("azorius"), "c:wu");
        assert_eq!(client.format_color_query("bant"), "c:gwu");
        assert_eq!(client.format_color_query("c:existing"), "c:existing");
    }

    #[test]
    fn test_format_comparison() {
        let client = ScryfallClient::new().expect("Failed to create client");

        assert_eq!(client.format_comparison("3"), "=3");
        assert_eq!(client.format_comparison(">=4"), ">=4");
        assert_eq!(client.format_comparison("<=2"), "<=2");
        assert_eq!(client.format_comparison(">1"), ">1");
    }
}
