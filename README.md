# Wasm Snake game

In order to use the game structure, you need to import it from a frontend application. Then get properties from an instance of the class.

## Usage
```typescript
import { Game } from "snake";

let game = Game.new();

game.x();
...
if(game.game_over()) {
    canvas.remove();
}
```

I've created [Wasm Snake game display](https://github.com/Shimon1995/Snake-Display) to abstract most of these fields.