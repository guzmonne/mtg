# Set Commands

The `mtg sets` command provides access to Magic: The Gathering set information and booster pack generation.

## Available Commands

- `list` - List all Magic sets
- `search <NAME>` - Search sets by name
- `get <CODE>` - Get specific set by code
- `booster <CODE>` - Generate a booster pack

## Set Listing

### Basic Listing

```bash
# List all sets (paginated)
mtg sets list

# List with custom page size
mtg sets list --page-size 10

# Get specific page
mtg sets list --page 2 --page-size 20
```

### Filtering by Name

```bash
# Sets with "Zendikar" in the name
mtg sets list --name "Zendikar"

# Sets with "Masters" in the name
mtg sets list --name "Masters"
```

### Filtering by Block

```bash
# Sets from Innistrad block
mtg sets list --block "Innistrad"

# Sets from Ravnica block
mtg sets list --block "Ravnica"
```

### Filtering by Type

```bash
# Core sets only
mtg sets list --type "core"

# Expansion sets
mtg sets list --type "expansion"

# Masters sets
mtg sets list --type "masters"

# Commander products
mtg sets list --type "commander"
```

## Set Search

Search for sets by name with partial matching:

```bash
# Find sets with "Dominaria" in the name
mtg sets search "Dominaria"

# Find Ravnica sets
mtg sets search "Ravnica"

# Find recent sets
mtg sets search "2024"
```

## Get Specific Set

Retrieve detailed information about a specific set:

```bash
# Get Khans of Tarkir
mtg sets get "KTK"

# Get Alpha
mtg sets get "LEA"

# Get Modern Horizons 3
mtg sets get "MH3"
```

## Booster Pack Generation

Generate virtual booster packs from specific sets:

```bash
# Generate Khans of Tarkir booster
mtg sets booster "KTK"

# Generate Innistrad booster
mtg sets booster "ISD"

# Generate recent set booster
mtg sets booster "BLB"  # Bloomburrow
```

## Output Examples

### Set List
```
 Code  Name                    Type        Block           Release Date 
 10E   Tenth Edition           core        Core Set        2007-07-13 
 2ED   Unlimited Edition       core        Core Set        1993-12-01 
 2X2   Double Masters 2022     masters     N/A             2022-07-08 
 2XM   Double Masters          masters     N/A             2020-08-07 
 3ED   Revised Edition         core        Core Set        1994-04-01 

Showing 5 sets (Page 1)
```

### Detailed Set View
```
=== Khans of Tarkir (KTK) ===

Name: Khans of Tarkir
Code: KTK
Type: expansion
Block: Khans of Tarkir
Release Date: 2014-09-26

Description: The first set in the Khans of Tarkir block, featuring the 
five three-color clans of Tarkir and their wedge-colored strategies.

Cards in Set: 269
Booster Available: Yes

Key Mechanics:
- Morph
- Delve
- Outlast
- Prowess
- Raid
```

### Booster Pack
```
**Booster Pack for KTK**

=== Mythic Rare ===
• Sorin, Solemn Visitor

=== Rare ===
• Siege Rhino

=== Uncommons ===
• Abzan Charm
• Jeskai Charm
• Mardu Charm

=== Commons ===
• Monastery Swiftspear
• Ponyback Brigade
• Dismal Backwater
• Jungle Hollow
• Scoured Barrens
• Swiftwater Cliffs
• Thornwood Falls
• Wind-Scarred Crag
• Blossoming Sands
• Bloodfell Caves

Total: 15 cards (1 Mythic, 1 Rare, 3 Uncommons, 10 Commons)
```

## Command Options

### List Options
- `--name <NAME>` - Filter by set name
- `--block <BLOCK>` - Filter by block name
- `--type <TYPE>` - Filter by set type
- `--page <NUM>` - Page number (default: 1)
- `--page-size <SIZE>` - Results per page (default: 20)

### Search Options
- `--page <NUM>` - Page number
- `--page-size <SIZE>` - Results per page

### Global Options
- `--api-base-url <URL>` - Custom API endpoint
- `--timeout <SECONDS>` - Request timeout
- `--verbose` - Detailed output

## Set Types

Common set types you can filter by:

- `core` - Core sets (e.g., Magic 2015)
- `expansion` - Regular expansion sets
- `masters` - Masters series (reprints)
- `commander` - Commander products
- `planechase` - Planechase sets
- `archenemy` - Archenemy sets
- `vanguard` - Vanguard cards
- `from_the_vault` - From the Vault series
- `spellbook` - Signature Spellbook series
- `premium_deck` - Premium Deck Series
- `duel_deck` - Duel Decks
- `draft_innovation` - Draft innovation sets
- `treasure_chest` - Magic Online Treasure Chests

## Popular Set Codes

### Recent Sets
- `BLB` - Bloomburrow (2024)
- `MH3` - Modern Horizons 3 (2024)
- `OTJ` - Outlaws of Thunder Junction (2024)
- `MKM` - Murders at Karlov Manor (2024)

### Classic Sets
- `LEA` - Limited Edition Alpha
- `LEB` - Limited Edition Beta
- `2ED` - Unlimited Edition
- `3ED` - Revised Edition
- `4ED` - Fourth Edition

### Popular Expansions
- `KTK` - Khans of Tarkir
- `RTR` - Return to Ravnica
- `ISD` - Innistrad
- `ZEN` - Zendikar
- `TSP` - Time Spiral

### Masters Sets
- `2X2` - Double Masters 2022
- `2XM` - Double Masters
- `UMA` - Ultimate Masters
- `A25` - Masters 25

## Booster Pack Notes

### Availability
Not all sets support booster generation. Sets that typically support it:
- Most expansion sets
- Core sets
- Some Masters sets

### Rarity Distribution
Standard booster packs typically contain:
- 1 Rare or Mythic Rare
- 3 Uncommons
- 10-11 Commons
- 1 Basic Land (in some sets)

### Special Cases
Some sets have unique booster structures:
- Masters sets may have different rarity distributions
- Some sets include special card types
- Older sets may have different pack compositions

## Tips and Tricks

### Finding Sets
```bash
# Find all Ravnica sets
mtg sets search "Ravnica"

# Find sets from a specific year
mtg sets list --name "2023"

# Find all Masters sets
mtg sets list --type "masters"
```

### Booster Generation
```bash
# Generate multiple boosters (run command multiple times)
for i in {1..3}; do
  echo "=== Booster $i ==="
  mtg sets booster "KTK"
  echo
done
```

### Set Research
```bash
# Get detailed info about a set before opening boosters
mtg sets get "KTK"

# Then generate booster
mtg sets booster "KTK"
```

---

Next: [Type Commands](types.md) | Back: [Card Commands](cards.md)