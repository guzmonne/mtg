# Type Commands

The `mtg types` command provides access to Magic: The Gathering type system information, including card types, subtypes, supertypes, and game formats.

## Available Commands

- `list` - List all card types
- `subtypes` - List all subtypes
- `supertypes` - List all supertypes  
- `formats` - List all game formats

## Card Types

### List All Types

```bash
# Get all card types
mtg types list
```

This returns all primary card types including:
- **Artifact** - Non-creature permanents with various effects
- **Battle** - New card type from March of the Machine
- **Creature** - Permanents that can attack and block
- **Enchantment** - Permanent magical effects
- **Instant** - Spells with immediate effects
- **Land** - Mana-producing permanents
- **Planeswalker** - Powerful ally permanents
- **Sorcery** - Spells cast at sorcery speed

### Example Output
```
=== Card Types ===

 Type 
 Artifact 
 Battle 
 Conspiracy 
 Creature 
 Enchantment 
 Instant 
 Land 
 Planeswalker 
 Sorcery 
 Tribal 

Total: 10 types
```

## Subtypes

### List All Subtypes

```bash
# Get all subtypes
mtg types subtypes
```

Subtypes are more specific classifications within each card type:

#### Creature Subtypes (Races and Classes)
- **Races**: Human, Elf, Goblin, Dragon, Angel, Demon, etc.
- **Classes**: Warrior, Wizard, Cleric, Rogue, Knight, etc.

#### Land Subtypes
- **Basic Types**: Plains, Island, Swamp, Mountain, Forest
- **Nonbasic Types**: Desert, Gate, Lair, etc.

#### Artifact Subtypes
- Equipment, Vehicle, Treasure, Food, etc.

#### Planeswalker Subtypes
- Jace, Chandra, Liliana, Gideon, Nissa, etc.

### Example Output
```
=== Subtypes ===

 Subtype 
 Advisor 
 Aetherborn 
 Angel 
 Antelope 
 Ape 
 Archer 
 Archon 
 Artificer 
 Assassin 
 Assembly-Worker 
 ...

Total: 347 subtypes
```

## Supertypes

### List All Supertypes

```bash
# Get all supertypes
mtg types supertypes
```

Supertypes are special designations that modify how cards work:

- **Basic** - Basic lands (Plains, Island, Swamp, Mountain, Forest)
- **Legendary** - Unique permanents (only one copy allowed)
- **Snow** - Cards that count as snow permanents
- **World** - Old supertype for unique enchantments

### Example Output
```
=== Supertypes ===

 Supertype 
 Basic 
 Legendary 
 Snow 
 World 

Total: 4 supertypes
```

## Game Formats

### List All Formats

```bash
# Get all game formats
mtg types formats
```

This returns all official Magic formats where cards can be legal:

#### Constructed Formats
- **Standard** - Most recent sets
- **Pioneer** - Sets from Return to Ravnica forward
- **Modern** - Sets from Eighth Edition forward
- **Legacy** - All sets with restricted list
- **Vintage** - All sets with power restrictions

#### Limited Formats
- **Draft** - Booster draft format
- **Sealed** - Sealed deck format

#### Casual Formats
- **Commander** - 100-card singleton multiplayer
- **Brawl** - Standard-legal Commander variant
- **Pauper** - Commons only
- **Penny Dreadful** - Budget format

### Example Output
```
=== Game Formats ===

 Format 
 Standard 
 Pioneer 
 Modern 
 Legacy 
 Vintage 
 Commander 
 Brawl 
 Pauper 
 Penny Dreadful 
 Draft 
 Sealed 

Total: 11 formats
```

## Detailed Information

### Understanding Card Types

#### Primary Types
Each card has exactly one primary type:
```bash
# Find creatures
mtg cards list --type "Creature"

# Find instants
mtg cards list --type "Instant"

# Find artifacts
mtg cards list --type "Artifact"
```

#### Multiple Types
Some cards have multiple types:
- **Artifact Creature** - Both artifact and creature
- **Legendary Creature** - Legendary supertype + creature type
- **Tribal Instant** - Tribal type + instant type

### Subtype Usage

#### Creature Subtypes
```bash
# Find all Dragons
mtg cards list --subtype "Dragon"

# Find all Warriors
mtg cards list --subtype "Warrior"

# Find Human Warriors
mtg cards list --subtype "Human,Warrior"
```

#### Land Subtypes
```bash
# Find all Islands
mtg cards list --subtype "Island"

# Find all basic lands
mtg cards list --supertype "Basic"
```

### Format Legality

When viewing individual cards, you'll see format legality:
```
Legality:
  Standard: Not Legal
  Pioneer: Legal
  Modern: Legal
  Legacy: Legal
  Vintage: Legal
  Commander: Legal
  Pauper: Not Legal
```

## Command Options

### Global Options
- `--api-base-url <URL>` - Custom API endpoint
- `--timeout <SECONDS>` - Request timeout
- `--verbose` - Detailed output

## Practical Examples

### Deck Building Research

```bash
# What creature types are available?
mtg types subtypes | grep -i "creature types"

# What formats exist for competitive play?
mtg types formats

# Find all legendary creatures
mtg cards list --supertype "Legendary" --type "Creature"
```

### Format Research

```bash
# Check what formats exist
mtg types formats

# Find cards legal in specific formats
mtg cards list --format "Modern"

# Research tribal strategies
mtg types subtypes | grep -i "tribal"
```

### Card Analysis

```bash
# Understand artifact subtypes
mtg types subtypes | grep -i "equipment\|vehicle"

# Research planeswalker types
mtg types subtypes | grep -i "planeswalker"

# Find basic land types
mtg types supertypes
mtg types subtypes | grep -i "plains\|island\|swamp\|mountain\|forest"
```

## Type System Rules

### Supertypes
- Can have multiple supertypes (e.g., "Legendary Snow Creature")
- Modify how the card functions
- "Basic" only applies to lands
- "Legendary" follows the legend rule

### Types  
- Each card has exactly one or more types
- Determine when and how the card can be played
- Affect what zones the card can exist in

### Subtypes
- Provide additional classification
- Enable tribal synergies
- Can have multiple subtypes
- Creature cards typically have race and/or class subtypes

## Advanced Usage

### Combining with Card Searches

```bash
# Find all legendary dragons
mtg cards list --supertype "Legendary" --type "Creature" --subtype "Dragon"

# Find equipment artifacts
mtg cards list --type "Artifact" --subtype "Equipment"

# Find tribal spells
mtg cards list --type "Tribal"
```

### Format-Specific Queries

```bash
# Research Standard-legal cards
mtg cards list --format "Standard"

# Find Commander-specific cards
mtg cards list --format "Commander"

# Research Pauper commons
mtg cards list --format "Pauper" --rarity "Common"
```

## Reference Tables

### Common Creature Types
| Race | Class | Hybrid |
|------|-------|---------|
| Human | Warrior | Zombie Warrior |
| Elf | Wizard | Angel Warrior |
| Goblin | Rogue | Dragon Wizard |
| Dragon | Knight | Vampire Noble |

### Land Types
| Basic | Nonbasic |
|-------|----------|
| Plains | Gate |
| Island | Desert |
| Swamp | Lair |
| Mountain | Locus |
| Forest | Mine |

---

Next: [Configuration](configuration.md) | Back: [Set Commands](sets.md)