use makepad_widgets::*;

use crate::{
    app::AppState, home::room_screen::RoomScreenWidgetExt
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::search_bar::SearchBar;

    import crate::home::room_screen::RoomScreen;
    import crate::home::welcome_screen::WelcomeScreen;

    ICON_NAV_BACK = dep("crate://self/resources/icons/navigate_back.svg")

    MainContent = {{MainContent}} {
        width: Fill, height: Fill
        flow: Down,
        show_bg: true
        draw_bg: {
            color: (COLOR_PRIMARY_DARKER)
        }
        align: {x: 0.0, y: 0.5}

        <SearchBar> {}

        welcome = <WelcomeScreen> {}
        rooms = <View> {
            align: {x: 0.5, y: 0.5}
            width: Fill, height: Fill
            room_screen = <RoomScreen> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MainContent {
    #[deref]
    view: View,
}

impl Widget for MainContent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // log!("Redrawing MainContent");
        let app_state = scope.data.get::<AppState>().unwrap();

        if let Some(room) = app_state.rooms_panel.selected_room.as_ref() {
            let displayed_room_name = room.name.clone().unwrap_or_else(|| format!("Room ID {}", &room.id));
            
            // Get a reference to the `RoomScreen` widget and tell it which room's data to show.
            self.view
                .room_screen(id!(room_screen))
                .set_displayed_room(cx, displayed_room_name, room.id.clone());

            self.view.view(id!(welcome)).set_visible(false);
            self.view.view(id!(rooms)).set_visible(true);
        } else {
            self.view.view(id!(welcome)).set_visible(true);
            self.view.view(id!(rooms)).set_visible(false);
            return self.view.draw_walk(cx, scope, walk);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}
