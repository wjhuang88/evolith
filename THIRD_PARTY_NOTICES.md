# Third-Party Notices

This file records third-party notices that Evolith must preserve when source or substantial
portions are incorporated or distributed. Dependency manifests and lockfiles remain the
authoritative software inventory.

## WalGit

- Project: WalGit
- Source: https://github.com/tobi/walgit
- Pinned revision for EVO-126-A: `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`
- License: MIT
- Copyright: Copyright (c) 2026 the walgit authors

The dependency pin and upgrade policy are documented in
`docs/reference/WALGIT-DEPENDENCY-BASELINE.md`.

### MIT License

Copyright (c) 2026 the walgit authors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


## bisync2

- Project: bisync2
- Source: https://github.com/de-vri-es/bisync2-rs
- Dependency version: `0.3.2`
- License choice used by Evolith: MIT (upstream offers MIT OR Apache-2.0)
- Copyright: Copyright (c) 2024 Jonas Maier
- Purpose: maintained implementation behind the local `bisync` compatibility shim required
  by WalGit's current gix dependency generation.

The Evolith shim only re-exports `bisync2`; it does not copy the upstream implementation.

### MIT License

Copyright (c) 2024 Jonas Maier

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
