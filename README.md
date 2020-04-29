# Yewtify

Vuetify components for Yew Framework.

## Usage

Add this crate to your dependencies list in `Cargo.toml`:

```
yewtify = { git = "https://github.com/yewstack/yewtify" }
```

And use components in `html!` macro:

```
html! {
    <yewtify::App>
        <yewtify::NavigationDrawer>
        </yewtify::NavigationDrawer>
    </yewtify::App>
}
```

## Add more components

Work in progress yet... Feel free to add your component.

To port a component that hasn't ported yet open this folder:

https://github.com/vuetifyjs/vuetify/tree/master/packages/vuetify/src/components

Choose a component you need and add it to `src/components` folder.
