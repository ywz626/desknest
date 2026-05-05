# TODOS

## Phase 2: Explorer desktop context menu integration

**What:** Add native Explorer desktop context menu integration for DeskNest actions.

**Why:** Match the KuDai Desktop-style entry point after overlay and real file moves are stable.

**Pros:** Users can right-click the Windows desktop background and access DeskNest actions like create box, settings, and exit.

**Cons:** Windows Shell integration is high-risk and should not block the first desktop overlay and file movement milestone.

**Context:** This should reuse Rust commands created for tray or box actions. It should not be implemented until desktop overlay behavior and move journal commands are stable.

**Depends on / blocked by:** Slice 1 overlay stability and Slice 2 move journal commands.

## Phase 2: OneDrive Desktop validation

**What:** Validate and support OneDrive-redirected Desktop for real file moves and undo.

**Why:** OneDrive can change filesystem behavior through placeholders, sync conflicts, delayed writes, and cloud-only files.

**Pros:** Improves reliability for common Windows setups where Desktop is redirected to OneDrive.

**Cons:** Adds a larger Windows QA matrix and may require special handling for placeholder hydration and sync conflicts.

**Context:** The first milestone can declare local Desktop only. This TODO tracks post-milestone hardening before broader release claims.

**Depends on / blocked by:** Slice 2 move journal and undo semantics.

## Phase 2: Multi-monitor and virtual desktop support

**What:** Add multi-monitor, per-monitor DPI, and Windows virtual desktop support.

**Why:** Desktop boxes are spatial UI; monitor identity and scaling determine whether layout survives real use.

**Pros:** Makes DeskNest usable on real multi-monitor desktop setups and reduces layout breakage.

**Cons:** Requires more desktop-layer QA and more complex coordinate handling.

**Context:** The layout model should keep monitor fields now, but behavior support waits until primary-monitor overlay works.

**Depends on / blocked by:** Slice 1 desktop attachment strategy passing on the primary monitor.

## Release: GitHub Actions installer pipeline

**What:** Add a GitHub Actions workflow to build and upload the Windows NSIS installer on version tags.

**Why:** Make open-source releases reproducible and easier to trust.

**Pros:** Release installers can be built from tagged source in CI instead of only from a local machine.

**Cons:** Requires maintaining Rust, Node, Tauri bundling, and Windows runner setup.

**Context:** Current manual release works. Automate after the desktop-native milestone stabilizes.

**Depends on / blocked by:** Existing Tauri build continuing to pass after desktop-native refactor.
