# domino
**domino** is a time-based task orchestration library with deterministic subtick precision and residue propagation.

## Nodes
Everything in **domino** is a node. Nodes are the essential building blocks that together form a behavior structure.

Consider a scenario where you want your code to greet the user with a "Hello!" message, wait 3 seconds, and then say "Goodbye!" and close the application. We have two actions that need to happen with a delay between them.

- Say "Hello!".
- Wait for 3 seconds.
- Say "Goodbye!".

Each of these three steps is a node.

`Action node -> Wait node -> Action node.`

- **Action nodes** execute the function attached to them.
- **Wait nodes** introduce a delay of a given duration.

```rust
let three_seconds = Duration::from_secs(3);

let mut main = sequence!(
    action!(say_hello),
    wait!(three_seconds),
    action!(say_goodbye)
);
```

Everything gets wrapped in a `Sequence` node that executes its children in order.

Nodes are driven by ticks. Each tick is a pulse carrying a delta time that propagates through the subsequent nodes.

```rust
main.tick(Duration::from_secs(2));
```

Ticking with a delta of 2 seconds causes the first node to execute. It prints "Hello!", returns `Success`, and since it is immediate it consumes zero delta, passing the full 2 seconds to the next node.

The `Wait` node receives those 2 seconds. Since it needs 3 seconds total, that is not enough. It returns `Pending`, consumes all remaining delta, and blocks execution for this tick.

Let's tick again!

```rust
main.tick(Duration::from_secs(2));
```

The first node is skipped since it already succeeded. The delta flows to the `Wait` node. The total accumulated time is now 4 seconds, which exceeds the 3-second delay. It returns `Success` and passes the leftover 1 second forward.

The final `Action` node prints "Goodbye!", completes with `Success`, and returns the 1 second of remaining delta, since its execution was immediate and consumed no time.

The entire flow completes with a status of `Success`.

> [!NOTE]
> Nodes will always allow themselves to be ticked. It is up to the caller to check the returned status and decide whether to continue ticking.

Consider the same example, but with 4 seconds passing in a single tick instead.

- The first node prints "Hello!", returns `Success`, and passes 4 seconds of remaining delta.
- The `Wait` node consumes 3 seconds and returns `Success` with 1 second remaining.
- The last node prints "Goodbye!" and returns `Success` with 1 second remaining.

Both cases (two ticks of 2 seconds or a single tick of 4 seconds) produce identical results: all nodes executed, 1 second of remaining delta. The behavior is deterministic regardless of tick frequency, as long as delta time is accurately provided.

> [!NOTE]
> Residue propagation ensures no time is lost between ticks.

## The macros

There are macros to simplify building behavior flows. `Sequence` is a binary node, meaning it takes exactly two children. This:

```rust
Sequence::new(
    Action::new(say_hello),
    Sequence::new(
        Wait::new(Duration::from_secs(3)),
        Action::new(say_goodbye),
    ),
)
```

Becomes this:

```rust
sequence!(
    action!(say_hello),
    wait!(Duration::from_secs(3)),
    action!(say_goodbye),
)
```

The verbose `Action::new(say_hello)` is represented as `action!(say_hello)`.

The macro recursively nests the binary `Sequence` nodes for you, so you can pass any number of children without worrying about the underlying structure.

It might not seem like much, but the difference becomes clear as soon as you start nesting more than a handful of nodes.

## Zero runtime overhead

**domino** has zero runtime overhead. Every node is a concrete struct and the behavior tree is fully resolved at compile time. No trait objects, no heap allocations, no dynamic dispatch.

Compositor nodes like `Sequence` and `Parallel` are generic over their children, so nesting three nodes produces a concrete type like `Sequence<Action<F>, Sequence<Wait, Action<G>>>`, a single stack-allocated value whose entire structure the compiler knows ahead of time.

The macros are what make this ergonomic. `Sequence` only takes two children; `sequence!` expands your flat list into the correctly nested binary tree automatically, and the compiler reduces it all to a tight inline call chain with no overhead.

The expressiveness lives in the macro; the performance guarantee lives in the type.
