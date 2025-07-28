# MTG Companion Watch - Current State

## Overview
The `mtg companion watch` command is a comprehensive real-time log parser for MTG Arena that provides tournament-level detail for every game action. It monitors game events and displays them in a user-friendly format with full card details from Scryfall.

**Platform Support**: This feature is only available on macOS and Windows, as MTG Arena is not available on Linux.

## Current Status: ✅ Feature Complete

### Core Features Implemented

#### 1. **Real-Time Log Monitoring**
- Automatic detection of newest MTG Arena log file
- Continuous monitoring with state persistence
- Resume from last position after restart
- Automatic log rotation handling
- Multi-line event parsing for both incoming (`<==`) and outgoing (`==>`) events

#### 2. **Comprehensive Event Processing**
- **Game State Messages** (`GREMessageType_GameStateMessage`)
  - Turn progression with phases and steps
  - Board state updates
  - Player status changes
  - Zone transfers
  - Game object tracking
  
- **Timer State Messages** (`GREMessageType_TimerStateMessage`)
  - Active player timer tracking
  - Inactivity timer monitoring
  - Warning thresholds
  - Elapsed time with millisecond precision

- **Business Events** (`LogBusinessEvents`)
  - Match analytics with detailed statistics
  - Match results with game duration
  - Opening hand information
  - Timer usage (rope) statistics

- **State Changes** (Unity logger events)
  - Game state transitions (Playing → MatchCompleted)
  - Match lifecycle tracking

#### 3. **Card Integration with Scryfall**
- Asynchronous card data fetching
- Card details displayed inline with actions:
  - Name, type, mana cost, CMC
  - Oracle text (truncated for readability)
  - Power/toughness for creatures
  - Loyalty for planeswalkers
- Context-aware card display (e.g., "Land played from Hand")

#### 4. **Detailed Action Tracking**
- **Mana Activation**: Shows which lands/sources are tapped
- **Spell Casting**: Displays mana costs with proper symbols
- **Ability Activation**: Shows ability costs and targets
- **Zone Transfers**: Tracks card movement with categories
- **Combat Actions**: Attack/block declarations

#### 5. **Comprehensive Annotation Processing**
- **Zone Transfers**: PlayLand, CastSpell, Draw, Discard, Resolve
- **User Actions**: Cast, Activate, Play, Attack, Block
- **Object Changes**: ID changes when cards transform
- **Combat**: Damage dealt between objects
- **Life Changes**: Gain/loss with visual indicators (💚/💔)
- **Phase/Step Transitions**: Detailed game flow tracking
- **Turn Management**: New turn notifications with active player
- **Permanent States**: Tap/untap events (⚡/♻️)
- **Object Removal**: Tracks destroyed/exiled objects
- **Effect Expiration**: Persistent effect end notifications

#### 6. **Match Analytics & Results**
- Post-game statistics:
  - Average response times
  - Priority counts (received/passed/responded)
  - Spell casting breakdown (auto-pay vs manual)
  - Board state peaks (max creatures/lands/artifacts)
- Match results:
  - Winner/loser with victory type
  - Game duration in turns and real time
  - Starting hands and mulligan information
  - Rope usage statistics

#### 7. **User Experience Features**
- Pretty-printed quest information with progress bars
- Rank tracking with win rates
- Course/event status with deck information
- Deck display with draw probabilities
- Mana curve visualization
- Visual indicators throughout (emojis for different event types)
- Verbose mode for debugging

## Architecture

### Component Structure
```
companion/watch/
├── mod.rs              # Main entry point and event routing
├── tailer.rs           # Log file monitoring and event extraction
├── parser.rs           # Event parsing and game state tracking
├── async_processor.rs  # Async card fetching from Scryfall
├── display.rs          # Output formatting
├── resolver.rs         # Card resolution (legacy)
├── state.rs            # Persistent state management
└── types.rs            # Data structures
```

### Key Design Decisions
1. **Channel-based Async Processing**: Synchronous event parsing with async card fetching
2. **State Tracking**: Maintains game state for diff-based updates
3. **Modular Event Handling**: Each event type has dedicated handlers
4. **Error Resilience**: Graceful degradation when card data unavailable

## Usage Examples

### Basic Usage
```bash
# Watch the newest log file
mtg companion watch

# Watch from the beginning of the log
mtg companion watch --from-beginning

# Watch with verbose output
mtg companion watch --verbose

# Watch a specific log file
mtg companion watch --log-path /path/to/log
```

### Example Output
```
📍 Turn 11 - Player 1's Main 1

⚡ Actions:
  🔴 Player 1 taps object #302 for mana
     📋 Plains
        Type: Basic Land — Plains
        Cost: Free
        Text: {T}: Add {W}.
        Context: Tapped for mana by Player 1
  
  📜 Player 1 activates ability 168992 on object #334
     Paying: {2}{W}{B}
     📋 Raffine's Tower
        Type: Land — Plains Island Swamp
        Cost: Free
        Text: Raffine's Tower enters the battlefield tapped...
        Context: Ability activated by Player 1

🎭 Zone Transfer: PlayLand
  Object #378 moved from Hand to Battlefield
     📋 Swamp
        Type: Basic Land — Swamp
        Cost: Free
        Text: {T}: Add {B}.
        Context: Land played from Hand

📊 Player Status Update:
  💔 Player 2: 20 → 17 life (-3)

⏱️ Active Timers:
  ⏰ Active Player: 15.2s elapsed
  💤 Inactivity: 0.0s elapsed
```

## Testing
- Tested with real MTG Arena log files
- Handles all major event types
- Graceful error handling for malformed events
- Performance tested with large log files (100MB+)

## Future Enhancements (Optional)
- Export match history to various formats
- Statistical analysis across multiple matches
- Deck performance tracking
- Integration with deck building tools
- Real-time win probability calculations

## Known Limitations
- Requires MTG Arena to be running and generating logs
- Card data depends on Scryfall API availability
- Some rare event types may not be fully parsed

## Conclusion
The companion watch command is now a comprehensive tool that provides professional-level match coverage for MTG Arena games. It successfully tracks every game action with full context and card details, making it invaluable for learning, reviewing matches, and understanding complex gameplay interactions.