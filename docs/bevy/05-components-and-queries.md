# Components, Queries, and the Art of Data Modeling

Every entity in a Bevy world is, on its own, nothing more than an identifier — a lightweight handle, essentially an integer, that carries no data and implies no behavior. What gives an entity meaning is the collection of components attached to it, and what gives a game life is the set of systems that query for those components and act on them. Understanding how to design components well, and how to reach for exactly the data you need through Bevy's query system, is arguably the most important skill you can develop as a Bevy programmer. It is the foundation on which everything else rests: rendering, physics, animation, audio, user interface, artificial intelligence, networking — all of these are, at their core, just components being queried by systems.

A component in Bevy is a Rust type — usually a struct or an enum — that has been marked with the `Component` derive macro. The syntax is pleasantly minimal. You write a struct, you derive `Component`, and suddenly that type can be attached to entities in the world. A position in two-dimensional space might look like this:

```rust
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

A velocity, similarly:

```rust
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
```

And a health value:

```rust
#[derive(Component)]
struct Health(f32);
```

Notice how each of these types captures exactly one idea. A position is where something is. A velocity is how fast and in what direction something is moving. Health is how much damage something can still absorb before it is destroyed. These are small, focused data types, and their smallness is not a limitation — it is the entire point. In an Entity Component System architecture (commonly abbreviated ECS), the power comes from composition. An entity that has both a `Position` and a `Velocity` is something that moves. An entity that has a `Position`, a `Velocity`, and `Health` is something that moves and can be damaged. An entity that has only `Position` is something that exists at a location but does nothing else. You build up complex game objects not by writing elaborate class hierarchies but by snapping together simple, orthogonal pieces of data like bricks.

This is where newcomers to ECS, particularly those with years of experience in object-oriented programming (OOP), tend to stumble. The OOP instinct tells you to create a single, all-encompassing type that holds everything a player needs — position, velocity, health, inventory, name, score, input state, animation frame, collision box, and perhaps a reference to the player's favorite color. In Bevy, that instinct will lead you astray. A monolithic `PlayerData` struct defeats the purpose of ECS in several important ways. First, it means that any system touching any aspect of the player must lock all of the player's data, even the fields it does not care about, which harms parallelism. Second, it means you cannot reuse the movement logic for non-player entities, because movement is tangled up with inventory and health and everything else. Third, it makes queries broader and less expressive than they ought to be. The entire philosophy of ECS rests on the idea that data should be separated by concern, not grouped by identity. The entity is the identity. The components are the concerns.

That said, components should not be so granular that they lose coherence. A `PositionX` and a separate `PositionY` component would be taking the principle too far, because the x and y coordinates of a position are not independent concerns — they are always read and written together, they always mean something as a pair, and splitting them apart would add complexity without adding any useful flexibility. The right level of granularity is the level at which a piece of data represents a single, self-contained aspect of an entity that at least one system will want to access independently of the entity's other aspects. Position qualifies. Velocity qualifies. Health qualifies. The combination of all three into a single struct does not, because there are many systems that care about position but not health, or about health but not velocity.

## Required Components and the Departure from Bundles

For much of Bevy's history, the primary way to spawn an entity with a predetermined set of components was through something called a Bundle. A bundle was a struct that derived the `Bundle` trait and contained several components as its fields, and when you passed it to the `spawn` method, Bevy would unpack it and attach each field as a separate component. Bundles served as a construction-time convenience: instead of adding `Transform`, `GlobalTransform`, `Visibility`, `InheritedVisibility`, and `ViewVisibility` one at a time, you could spawn a `SpatialBundle` and get all of them at once. This pattern was useful, but it introduced a persistent source of confusion. Bundles existed only at spawn time. They were not components themselves, and they did not participate in the query system. You could not write `Query<&SpatialBundle>` to find all entities that had been spawned with a spatial bundle, because the bundle disappeared the moment its contents were unpacked. New users frequently tried to query for bundles and were baffled when it did not work, or worse, they confused bundles with components and struggled to understand the conceptual boundary between the two.

Bevy 0.15 introduced a cleaner solution called Required Components. The idea is elegant: instead of defining a separate bundle type that groups components together for spawning, you annotate a component itself with the other components it needs in order to function correctly. The annotation uses the `require` attribute:

```rust
#[derive(Component)]
#[require(Transform, Visibility)]
struct Player;
```

When you spawn an entity with the `Player` component, Bevy automatically ensures that `Transform` and `Visibility` (along with their own transitive requirements) are also present on that entity. If you do not supply explicit values for the required components, Bevy uses their `Default` implementations. If you do supply values, your values take precedence. The beauty of this approach is that `Player` is still a real component — it lives on the entity, it can be queried for, it can be added and removed dynamically — and the dependency information is declared right next to the type that needs it, rather than in a separate bundle struct that exists off to the side and participates in none of the runtime systems.

Required components also compose naturally. If `Transform` itself requires `GlobalTransform`, and `Visibility` requires `InheritedVisibility` and `ViewVisibility`, then requiring `Transform` and `Visibility` on `Player` transitively pulls in all of those dependencies without the `Player` type needing to know about them. This is a substantial improvement over bundles, where you had to manually include every component in the bundle struct, and where forgetting one would lead to subtle runtime bugs rather than compile-time guarantees. With the introduction of required components, bundles were deprecated, and the migration path is straightforward: replace your bundle struct with a component that uses `#[require(...)]` to declare its dependencies, and enjoy the fact that your type now works both at spawn time and at query time.

You can also provide default values for required components using a closure. For instance, if you want every `Player` to start with a specific transform rather than the `Default` one, you can write `#[require(Transform(|| Transform::from_xyz(0.0, 1.0, 0.0)))]`. This keeps the default-value logic close to the component that cares about it, rather than scattering it across various spawn sites throughout your codebase.

## Queries: Reaching into the World

If components are the atoms of a Bevy world, queries are the hands that reach in and manipulate them. A query is how a system declares which components it wants to access and, optionally, which entities it wants to include or exclude based on the presence or absence of certain components. The syntax is `Query<D, F>`, where `D` is the data you want to read or write, and `F` is an optional set of filters that narrow down which entities match. When you omit the filter parameter, every entity that has the requested components will match.

The simplest query reads a single component immutably. A system that prints the position of every entity in the world might look like this:

```rust
fn print_positions(query: Query<&Position>) {
    for position in &query {
        println!("x: {}, y: {}", position.x, position.y);
    }
}
```

The `&Position` in the query parameter means "give me a shared, read-only reference to the Position component." Bevy's scheduler uses this information to determine which systems can run in parallel: two systems that both read `Position` can run simultaneously, but a system that reads `Position` and a system that writes `Position` cannot, because the writer needs exclusive access. This is why distinguishing between `&Position` and `&mut Position` in your queries matters — it is not merely a Rust formality, it is how Bevy reasons about data access at the scheduling level.

To write to a component, you use `&mut`:

```rust
fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
```

Here, the query asks for both `Position` (mutably) and `Velocity` (immutably) on the same entity. Only entities that have both components will appear in the iteration. An entity with `Position` but no `Velocity` will be silently skipped, as will an entity with `Velocity` but no `Position`. This is the natural filtering behavior of queries: by specifying what data you want, you implicitly specify which entities you are interested in.

Sometimes, though, you want to filter entities based on the presence or absence of a component without actually reading that component's data. This is where query filters come in. The two most common filters are `With<T>` and `Without<T>`. A system that moves only player entities, for example, might look like this:

```rust
fn move_players(mut query: Query<(&mut Position, &Velocity), With<Player>>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
```

The `With<Player>` filter means "only match entities that have the `Player` component," but since `Player` does not appear in the data portion of the query, the system does not actually read the `Player` component's value. It uses `Player` purely as a tag to narrow the set of matching entities. Conversely, `Without<Enemy>` would exclude all entities that have the `Enemy` component. You can combine filters freely: `Query<&Position, (With<Player>, Without<Invisible>)>` matches entities that have `Position` and `Player` but not `Invisible`. This composability is part of what makes the query system so expressive; you can describe precisely the subset of entities you care about, and Bevy handles the rest.

It is worth pausing here to appreciate the analogy between Bevy's query system and a relational database. If you think of the Bevy world as a database, then entities are rows, components are columns, and a query is a `SELECT` statement. `Query<&Position>` is like `SELECT position FROM entities WHERE position IS NOT NULL`. Adding `With<Player>` is like appending `AND player IS NOT NULL` to the `WHERE` clause. Adding `Without<Enemy>` appends `AND enemy IS NULL`. The data portion of the query (`&Position`, or `(&Position, &Velocity)`) determines which columns appear in the result set, and the filter portion determines which rows are included. This is not a perfect analogy — Bevy stores data in archetypes rather than in a traditional row-or-column store, and there are no joins in the relational sense — but as a mental model for understanding what queries do, it is remarkably useful. You are asking a question of the world, and the query system gives you back exactly the answer.

When you know that exactly one entity should match your query — for instance, if you have a single camera, or a single player, or a single score display — you can use `query.single()` instead of iterating. In versions of Bevy prior to 0.16, the `single` method would panic at runtime if zero or more than one entity matched the query, which led to crashes that were easy to trigger accidentally and hard to debug. As of Bevy 0.16, `single()` returns a `Result`, which means you must handle the possibility that the query matched zero entities or more than one entity. This is a small change in syntax but a significant improvement in robustness, because it forces you to think about edge cases rather than assuming the world is in the exact state you expect. A typical usage might look like this:

```rust
fn track_player(
    player_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let Ok(player_position) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    camera_transform.translation.x = player_position.x;
    camera_transform.translation.y = player_position.y;
}
```

The early returns ensure that the system gracefully does nothing if the player or camera is missing, rather than bringing down the entire application with a panic.

## Reactive Queries: Changed, Added, and AssetChanged

Not every system needs to run over every matching entity every frame. Sometimes you only want to act on entities whose data has actually changed since the last time the system ran. Bevy provides change detection filters for exactly this purpose. The `Changed<T>` filter matches entities whose `T` component has been mutably dereferenced since the last time the current system executed. Note the subtlety: it detects mutable access, not necessarily a change in value. If a system obtains a `&mut Position` and writes the same value back, Bevy will still consider it changed, because Bevy tracks access at the borrow level, not the value level. This is a pragmatic tradeoff — comparing old and new values for every component every frame would be prohibitively expensive — and in practice it works well, because most systems that take mutable references to a component do so because they intend to modify it.

The `Added<T>` filter is closely related but subtly different. It matches entities that have had the `T` component added to them since the last time the current system ran. This is useful for initialization logic: when a new entity enters the world with a `Health` component, you might want to create a health bar user-interface element for it, and `Added<Health>` lets you detect that event without maintaining your own bookkeeping.

A system using change detection might look like this:

```rust
fn on_position_changed(query: Query<(Entity, &Position), Changed<Position>>) {
    for (entity, position) in &query {
        println!("Entity {entity:?} moved to ({}, {})", position.x, position.y);
    }
}
```

This system will do nothing on frames where no entity's position was mutably accessed, which can be a significant performance win in large worlds where most entities are stationary most of the time.

Bevy 0.16 extended the change detection concept to assets with the `AssetChanged` filter. Assets in Bevy — textures, meshes, audio clips, materials, and so on — live in a separate storage system from components, but they are often referenced by components via typed handles. When an asset is modified or reloaded (for instance, during hot-reloading of a texture during development), you may want systems to respond to that change and update their visual representation or recalculate derived data. The `AssetChanged<T>` filter makes this possible without requiring you to manually track asset versions or poll for differences. You simply write a query that filters on `AssetChanged<StandardMaterial>`, for example, and Bevy will ensure your system only iterates over entities whose material handle points to an asset that has been modified. This is a natural extension of the reactive query philosophy — instead of polling the world for changes, you declare what changes you care about, and the engine delivers only the relevant updates.

Together, `Changed`, `Added`, and `AssetChanged` form a toolkit for building reactive systems that respond efficiently to mutations in the world. They encourage a style of programming where each system does the minimum amount of work necessary, touching only the data that has actually been updated, rather than brute-forcing over every entity every frame. This matters not just for performance but for clarity: a system filtered on `Changed<Health>` communicates its intent — "I respond to health changes" — far more clearly than one that iterates over all entities with health and internally checks whether anything has changed.

The interplay between components and queries is, in many ways, the heartbeat of a Bevy application. Components define the shape of your data, and queries define the paths that systems take through that data. A well-designed Bevy codebase will have many small components, each representing a single facet of an entity, and many focused queries, each reaching for exactly the data one system needs to do its job. The result is a codebase that is modular almost by accident — because each system is coupled only to the components it queries, you can add, remove, and reorganize systems without cascading changes, and you can reuse components across different entity types without modification. A `Velocity` component works the same whether it is attached to a player, a bullet, a particle, or a cloud drifting across the sky.

This is the art of data modeling in Bevy: resisting the urge to bundle everything together, embracing the composability of small types, and trusting the query system to connect the right data to the right logic at the right time. It takes some adjustment if you are coming from an object-oriented background, where the instinct is to organize code around nouns — Player, Enemy, Projectile — each with their own methods and encapsulated state. In Bevy, you organize code around verbs — movement, collision, rendering, damage — and each verb reaches across entity types to operate on whichever entities have the relevant data. The nouns still exist as entities, but they are defined not by a class in a hierarchy but by the unique combination of components they carry. An entity is what it has, and a system does what its query lets it see.
