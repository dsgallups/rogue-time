Things to know

- for colliders in bevy, do not use ColliderConstructorHierachy::new(ColliderConstructor::ConvexDecompositionFromMesh)
  - it collides, but the collider doesnt ever point to the player. WTF.


Therefore, everything with portal should manually include a `ColliderConstructor` in the blender file.

Not the case for time pickups



- there is a condition for LevelCountdown's completion.
We are using an Update system intentionally because we need these all to run before the Ui Update occurs. Not guaranteed with a trigger.
