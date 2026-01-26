---
title: "The Boon of Dataflow"
description: "What time manipulation games, hardware circuits, and factory sims taught me about programming."
sortOrder: 0
tags: ["boon", "dataflow", "games", "books"]
---

![Boon](/images/blog/boon-ascii-logo.png)

Do you know what the book [*Lucid, the Dataflow Programming Language*](/shelf#lucid-dataflow-programming) and games like [*Braid*](/shelf#braid), [*TimeShift*](/shelf#timeshift), and [*Katana ZERO*](/shelf#katana-zero) have in common? **Time**. The human desire to control time — to revert mistakes or beat death — is displayed everywhere in art across history. But our brains cannot experience multiple "nows." And you actually don't want such an ability — it would fragment your identity like it does to Senua in [*Hellblade*](/shelf#hellblade). Or maybe we can copy ourselves to prevent fragmentation? Think about it while playing [*SOMA*](/shelf#soma) or watching *The Prestige*. Don't want to clone yourself? Maybe AI will help you, like in [*Detroit: Become Human*](/shelf#detroit-become-human), to do multiple things instead of you…

```elm
n   = 1 fby n + 1                  -- stream: 1, 2, 3, ...
fac = 1 fby (fac * (n + 1))        -- factorial stream
sum = first(x) fby sum + next(x)   -- running sum
```
*[Lucid](https://en.wikipedia.org/wiki/Lucid_(programming_language)) — time as infinite streams, "fby" means "followed by"*

But with the current limitations of our minds, we pretend the world is waiting for us and everything goes step-by-step. We design our programming languages and databases for such one "now." However, this simplification fights reality a lot, as described in [*Designing Data-Intensive Applications*](/shelf#designing-data-intensive-applications).

I think one solution is hinted at near the end of that book — actors and streams. Or let's say dataflow. We'll be able to read more about it in the second edition of [*Designing Data-Intensive Applications*](/shelf#designing-data-intensive-applications-2nd) (expected early 2026), but in the meantime, play [*Factorio*](/shelf#factorio) or [*Desynced*](/shelf#desynced) to get an idea how data — or rather, resources — can flow.

![Factorio train system](/images/blog/factorio-train.jpg)
*Artwork from [Factorio](https://www.factorio.com/) by Wube Software. [Press kit](https://www.factorio.com/support/press-kit).*

Then books like [*Flow-Based Programming*](/shelf#flow-based-programming), [*Lucid*](/shelf#lucid-dataflow-programming), or [*Domain Modeling Made Functional*](/shelf#domain-modeling-made-functional) show you how to build your factories with code. Notice the similarity with hardware circuits. Everything leads to Boon and BoonComputer. But we have to start building BoonComputer from simple things first — maybe draw inspiration from [*Computer Engineering for Big Babies*](/shelf#computer-engineering-for-big-babies).

The simplest dataflow is an immutable tree — everything flows one direction, like blood in our vessels or water in river deltas. But in reality, almost nothing is acyclic. Blood returns to our hearts, rain falls on mountains, car engines adjust fuel based on exhaust sensors, users trigger events in browser DOM trees according to what they see on the website. Loops are everywhere. You can start gently — learn how latches work in [*Computer Engineering for Babies*](/shelf#computer-engineering-for-babies), then play [*Portal*](/shelf#portal) to think outside the box and create loops and shortcuts. Lastly, [*COCOON*](/shelf#cocoon) can intuitively teach you what recursion is — without any text.

![COCOON - worlds within worlds](/images/blog/cocoon-worlds.gif)
*GIF from [COCOON](https://www.cocoongame.com/) by Geometric Interactive, published by Annapurna Interactive.*

Recursion is beautiful, but also a perfect example of "easy to demo, hard to reason about" once systems grow.

Boon uses `button: LINK` and `new_button() |> LINK { button }` to create a one-directional "portal" going against the flow. And also the `FLUSH` keyword — like old postal pneumatic tubes handling express delivery, running faster than regular mail. It's mostly designed for short-circuiting: when something bad happens, you need to deliver the message about the problem now. Read more about such Railway-Oriented patterns in [*Domain Modeling Made Functional*](/shelf#domain-modeling-made-functional).

It would be hard to operate such railways and pipelines without any real-time maps. Visualizations are crucial for dataflows — all Factorio-like games have advanced maps and charts showing various statistics and logistics. However, making cross-platform visualizers (or apps in general), especially ones that should work in both browser and desktop, is still a major problem even today.

![Factorio electric statistics](/images/blog/factorio-electric.png)
*Screenshot from [Factorio](https://www.factorio.com/) by Wube Software. [FFF-337](https://www.factorio.com/blog/post/fff-337).*

I hope the RayBox project will help with rendering — no slow HTML, no [*Layers of Fear*](/shelf#layers-of-fear) packaged as WebViews, no [*Happy Game*](/shelf#happy-game) of writing platform-specific renderers. Then we should make it pretty by drawing inspiration from [*Refactoring UI*](/shelf#refactoring-ui) and many games with intuitive UIs. The book [*elm-ui: The CSS Escape Plan*](/shelf#elm-ui-guide) will help with the RayBox layout API. Both MoonZoon and Boon are already drawing inspiration from it.

![Shape Up book cover](/images/blog/shapeup-cover.jpeg)
*[Shape Up](https://basecamp.com/shapeup) by Ryan Singer, Basecamp.*

This will be painful to implement. At least I can [*Shape Up*](/shelf#shape-up) the work into manageable cycles and [*Rework*](/shelf#rework) my approach when needed. I hope I'll stay [*In Sound Mind*](/shelf#in-sound-mind) and then [*Heal*](/shelf#heal) by writing nice Boon code. But [*Shift Happens*](/shelf#shift-happens), and I expect to have [*Little Nightmares*](/shelf#little-nightmares) while working on it — hopefully I won't get [*Lost in Random*](/shelf#lost-in-random) (even though I like Tim Burton-esque stories).
