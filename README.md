# Yewtify

Vuetify components for the [Yew framework](https://github.com/yewstack/yew).

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

## Already available components

- [x] **App** (v-app)
- [x] **NavigationDrawer** (v-navigation-drawer)
- [x] **List** (v-list)
- [x] **ListItem** (v-list-item)
- [x] **ListItemAction** (v-list-item-action)
- [x] **ListItemAvatar** (v-list-item-avatar)
- [x] **ListItemContent** (v-list-item-content)
- [x] **ListItemTitle** (v-list-item-title)

## Adding additional components

This is still a work in progress... feel free to add additional components.

To port a component that hasn't been ported yet open this folder:

https://github.com/vuetifyjs/vuetify/tree/master/packages/vuetify/src/components

Choose a component you need and add it to the `src/components` directory.
