---
title: Itemphobia
date: 2025-12-25
tags: [minecraft, fabric, java]
links: [https://github.com/smit4k/Itemphobia]
description: "A client side Minecraft fabric mod that lets you blacklist items that you do not want to pick up from the ground"
---

This is a Minecraft fabric mod that allows you to blacklist items that you do
not want to pick up from the ground. I made this because I realized that I was
constantly picking up random junk like rotten flesh, cobblestone and other
items that I did not want. It got so annoying that I checked if there was a mod
that could blacklist items, and there apparently wasn't one, so I took the
opportunity to make one myself.

:::info Download Links
The mod can be downloaded from [Modrinth](https://modrinht.com/mod/itemphobia) and [Github](https://github.com/smit4k/itemphobia/releases)
:::

## Usage

Once you have installed the mod, you can press `I` in-game to open the
Itemphobia Blacklist GUI. From there, you search for items to blacklist and can
add or remove items to the blacklist by pressing the green '+' or red '-'
buttons respectively. Once an item is blacklisted, it can no longer be picked
up from the ground.

## How it Works

### Blacklist

Blacklisted items are stored in a `JSON` file located in
`config/itemphobia.json`. I implemented a `HashSet` to store the blacklisted
items in memory for efficient lookup as well as to avoid frequent I/O
operations. This allowed me to make simple methods for adding, removing and
checking whether an item is blacklisted.

Here are snippets from
[ItemphobiaConfig.java](https://github.com/smit4k/Itemphobia/blob/main/src/main/java/codes/smit/config/ItemphobiaConfig.java)
to show how the blacklist works.

Blacklisted items are stored internally as `ResourceLocation`s. This avoids
storing direct `Item` references (which could result in serialization or
deserialization issues) and keeps the blacklist stable across reloads.

```java
private static final Set<ResourceLocation> blacklistedItems = new HashSet<>();
```

Adding an item to the blacklist involves getting its `ResourceLocation` and
saving the blacklist only if the item has been added as to prevent unnecessary
file writes.

```java
public static void addToBlacklist(Item item) {
    ResourceLocation id = BuiltInRegistries.ITEM.getKey(item);
    if (blacklistedItems.add(id)) {
        save();
    }
}
```

Checking if an item is blacklisted is as simple as checking whether its
`ResourceLocation` appears in `blacklistedItems`

```java
public static boolean isBlacklisted(Item item) {
    ResourceLocation id = BuiltInRegistries.ITEM.getKey(item);
    return blacklistedItems.contains(id);
}
```

This method is called during item collisions, so using a `HashSet` ensures
constant-time lookups during gameplay.

### Preventing Blacklisted Item Pickup

For the actual logic of not picking up blacklisted items, I used
[Mixins](https://github.com/SpongePowered/Mixin) to inject code when the player
touches an item entity. Take a look at this snippet from
[ItemPickupMixin.java](https://github.com/smit4k/Itemphobia/blob/main/src/main/java/codes/smit/mixin/ItemPickupMixin.java#L11)

```java
@Mixin(ItemEntity.class)
public class ItemPickupMixin {

    @Inject(method = "playerTouch", at = @At("HEAD"), cancellable = true)
    private void onPlayerTouch(Player player, CallbackInfo ci) {
        ItemEntity self = (ItemEntity)(Object)this;
        if (ItemphobiaConfig.isBlacklisted(self.getItem().getItem())) {
            ci.cancel();
        }
    }
}
```

This Mixin checks if a player touches an item entity, and if the item is a
blacklisted item, it cancels the pickup event, effectively preventing the
player from picking up the blacklisted item.
