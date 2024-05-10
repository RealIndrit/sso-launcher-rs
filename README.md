# SSO Launcher (Rust) (Based on [SimplyLaunch](https://github.com/vars1ty/SimplyLaunch))

## What is it?
Simply put, it is a heavily stripped down launcher for `Star Stable Online` with less bloat. A bit scuffed Rust code maybe... The amount of shit in the official launcher is nothing short of astonishing. And I personally do not wanna be within 10 feet radius of Electron based bloat programs if it can be avoided.

## Why?

I'll just use the original authors' description. It summarizes it pretty well:

_Because I don't want to sit and wait for their horrible pasta-coded launcher takes its sweet time to load useless stuff, like:_
- Star Coins
- Stable Care
- Offers
    - No joke, they got a whole damn subsection in their API just for this
- A whole-ass video
- Useless CSS which, by the way, **you'll only look at for 5 seconds until the game launches, nobody sits and stares at your Windows 8-like clustered launcher**
- A billion different libraries, many of which they could easily have re-created themselves.
    - Like grabbing your Machine ID, how can a million-dollar company fail to implement something so basic, in a launcher that's only exported for **2** platforms?
    - You only have to find a way of grabbing the ID on 2 separate platforms, and don't worry about Linux since WINE/Proton will to 99% work just fine with fetching it too.

_In conclusion; their launcher is a bloated, slow and utter mess. Just extract the asar and check for yourself, I'd be surprised if you came back with any sanity left._

## Does it implement everything the actual launcher has?
No, not even close to it. It has no store, no news, nothing of that useless shit. It will just do what's needed to download/update/launch the game, nothing less, nothing more.

Current features:
* Launch game
  * Set Language
  * Set custom game arguments (Not available by default on official launcher, use with caution)
* Checking game status
* Update/Install game (WIP)
* Directly download official launcher

## Where's the UI?
There's none, it's a terminal application. Terminal is more than enough for this use case, if you genuially believe you need a UI, just use the official launcher...

## Does this break the TOS?
I do not know specifically, so I will say `YES`. So use with caution and do not do anything stupid.

## How do I use it?

Run the executable in the terminal:
`(EXE) --help` and use that massive brain for the rest :P

## Disclaimer
1. I am in no way associated with Star Stable Entertainment AB.
2. I do not endorse using this in any way.
3. Anything you do, using this launcher, is sole the responsibility of you as the user. I do not take and responsibility for what it's used for.