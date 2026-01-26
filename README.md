# About

This app is designed to create a number of useful apps that can deploy to native, and eventually deploy to a web app using trunk and wasm. 

## Testing locally

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

## Web Locally

> [!IMPORTANT]
> This doesn't currently work due to the rt-multi-thread feature of tokio, which is strictly not supported in wasm. This can apparently be worked around if I can figure out how to spawn an executor within the thread, rather than marking the whole main function as async, but I barely know how to get out of this hole for now. 

The current signature will eventually drop the rt-multi-thread feature, but this is required currently for the async task.

```
tokio = { version = "1.48.0", features = ["rt", "net", "time", "macros", "rt-multi-thread"] }
```

Additionally, there are some references within the index.html template that refer to icons that have been replaced. These need to be either updated, or removed in order to get a reasonable build error using trunk serve. 

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

## Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.
>
> If you renamed the `main` branch to something else (say you re-initialized the repository with `master` as the initial branch), be sure to edit the github workflows `.github/workflows/pages.yml` file to reflect the change
> ```yml
> on:
>   push:
>     branches:
>       - <branch name>
> ```