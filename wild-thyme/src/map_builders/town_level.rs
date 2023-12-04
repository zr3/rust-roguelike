use rltk::{RandomNumberGenerator, RGB};
use specs::World;

use crate::{
    components::Position,
    map::{Map, TileType},
    map_builders::common::{apply_horizontal_tunnel, apply_room_to_map},
    rect::Rect,
    spawners,
};

use super::{
    common::{apply_tile_to_map, release_drunk},
    MapBuilder,
};

pub struct TownLevelBuilder {
    map: Map,
    starting_position: Position,
    start_room: Rect,
    pond_room: Rect,
    cake_room: Rect,
    portal_room: Rect,
    secret_room: Rect,
}

impl MapBuilder for TownLevelBuilder {
    fn build_map(&mut self) {
        for i in self.map.width as usize..self.map.tiles.len() - self.map.width as usize {
            if i % self.map.width as usize > 0
                && i % (self.map.width as usize) < self.map.width as usize - 1
                && (i % 13 == 0
                    || i % 17 == 0
                    || i % 31 == 0
                    || i & 43 == 0
                    || i % 11 == 0
                    || i % 19 == 0)
            {
                self.map.tiles[i] = TileType::Floor;
            }
        }

        self.start_room = Rect::new(35, 17, 10, 10);
        apply_room_to_map(&mut self.map, &self.start_room);
        self.map.rooms.push(self.start_room);

        self.pond_room = Rect::new(15, 15, 20, 20);
        apply_room_to_map(&mut self.map, &self.pond_room);
        self.map.rooms.push(self.pond_room);
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..30 {
            release_drunk(&mut self.map, (25, 25), 16, &mut rng, 10, TileType::Water);
        }

        self.cake_room = Rect::new(30, 30, 15, 10);
        apply_room_to_map(&mut self.map, &self.cake_room);
        apply_tile_to_map(&mut self.map, &Rect::new(35, 37, 5, 2), TileType::JudgeCake);
        apply_tile_to_map(
            &mut self.map,
            &Rect::new(35, 35, 5, 1),
            TileType::IngredientTable,
        );
        self.map.rooms.push(self.cake_room);

        self.portal_room = Rect::new(52, 20, 10, 6);
        apply_room_to_map(&mut self.map, &self.portal_room);
        apply_horizontal_tunnel(&mut self.map, 45, 52, 20);
        self.map.rooms.push(self.portal_room);
        let start_center = self.portal_room.center();
        let start_center_idx = self.map.xy_idx(start_center.0, start_center.1);
        self.map.tiles[start_center_idx] = TileType::DownStairs;

        self.secret_room = Rect::new(1, 20, 10, 5);
        apply_room_to_map(&mut self.map, &self.secret_room);
        apply_horizontal_tunnel(&mut self.map, 1, 13, 20);

        let start = self.start_room.center();
        self.starting_position = Position {
            x: start.0,
            y: start.1,
        };
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        spawners::npc(
            ecs,
            40,
            18,
            rltk::to_cp437('☺'),
            RGB::from_hex("#805010").expect("hardcoded"),
            "MYSTERIOUS FIGURE",
            vec![
                "been dark out here lately...".to_string(),
                "word is, there's good THYME deep in the forest".to_string(),
                "...".to_string(),
                "I heard ancient lizards don't like ROCKS..".to_string(),
                "FRIENDLY birds can help a person out..".to_string(),
                "walk around enough, you'll see some TRAPS out there".to_string(),
                "I'll give you something great if you can find me THYME!".to_string(),
                "TREE PORTALS will get you deeper into the forest..".to_string(),
                "don't get too HUNGRY...".to_string(),
            ],
        );
        let cake_center = self.cake_room.center();
        spawners::npc(
            ecs,
            cake_center.0 - 1,
            cake_center.1,
            rltk::to_cp437('☺'),
            RGB::from_hex("#a05010").expect("hardcoded"),
            "MR HOLLYWOOD",
            vec![
                "the GREAT WOODY BAKE OFF is coming soon!".to_string(),
                "help us BAKE a CAKE".to_string(),
                "my TASTE is absolutely exquisite".to_string(),
            ],
        );
        spawners::npc(
            ecs,
            cake_center.0 + 1,
            cake_center.1,
            rltk::to_cp437('☺'),
            RGB::from_hex("#805050").expect("hardcoded"),
            "MS GOODBERRY",
            vec![
                "I love CAKE of all shapes and sizes".to_string(),
                "we will JUDGE your CAKE when the time comes!".to_string(),
            ],
        );
        spawners::npc(
            ecs,
            cake_center.0,
            cake_center.1 + 1,
            rltk::to_cp437('☺'),
            RGB::from_hex("#807010").expect("hardcoded"),
            "SIR FIELDS",
            vec![
                "CAKE is made of several ingredients!".to_string(),
                "FLOUR will hold it together".to_string(),
                "MILK will help it mix".to_string(),
                "FAT will moisturize it".to_string(),
                "EGGS will keep it stable".to_string(),
                "SWEET will improve the taste".to_string(),
                "OTHERS will add a twist!".to_string(),
                "THYME will bring it all together".to_string(),
            ],
        );
        spawners::spawn_treeportal(ecs, &self.portal_room);

        // spawn doubled secret room items, (1, 20) to (11, 25)
        spawners::items::cake_knife(ecs, 3, 20);
        spawners::items::cake_knife(ecs, 3, 20);
        spawners::items::bark_armor(ecs, 4, 20);
        spawners::items::bark_armor(ecs, 4, 20);
        spawners::items::healing_herbs(ecs, 5, 20);
        spawners::items::healing_herbs(ecs, 5, 20);
        spawners::items::pos_meat(ecs, 6, 20);
        spawners::items::pos_meat(ecs, 6, 20);
        spawners::items::pos_milk(ecs, 7, 20);
        spawners::items::pos_milk(ecs, 7, 20);
        spawners::items::pos_egg(ecs, 8, 20);
        spawners::items::pos_egg(ecs, 8, 20);
        spawners::items::pointy_stick(ecs, 9, 20);
        spawners::items::pointy_stick(ecs, 9, 20);

        spawners::items::mushroom(ecs, 3, 21, "TEST".to_string(), -10, 30);
        spawners::items::mushroom(ecs, 3, 21, "TEST".to_string(), -10, 30);
        spawners::items::friendly_crow(ecs, 4, 21);
        spawners::items::friendly_crow(ecs, 4, 21);
        spawners::items::friendly_eagle(ecs, 5, 21);
        spawners::items::friendly_eagle(ecs, 5, 21);
        spawners::items::goodberry(ecs, 6, 21);
        spawners::items::goodberry(ecs, 6, 21);
        spawners::items::thyme(ecs, 7, 21);
        spawners::items::thyme(ecs, 7, 21);

        spawners::items::rock(ecs, 3, 22);
        spawners::items::rock(ecs, 3, 22);
        spawners::items::dart_gun(ecs, 4, 22);
        spawners::items::dart_gun(ecs, 4, 22);
        spawners::items::sparkling_powder(ecs, 5, 22);
        spawners::items::sparkling_powder(ecs, 5, 22);
        spawners::items::confusion_scroll(ecs, 6, 22);
        spawners::items::confusion_scroll(ecs, 6, 22);
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&mut self) -> Position {
        self.starting_position.clone()
    }
}

impl TownLevelBuilder {
    pub fn new() -> TownLevelBuilder {
        TownLevelBuilder {
            map: Map::new(1),
            starting_position: Position { x: 0, y: 0 },
            start_room: Rect::new(0, 0, 0, 0),
            pond_room: Rect::new(0, 0, 0, 0),
            cake_room: Rect::new(0, 0, 0, 0),
            portal_room: Rect::new(0, 0, 0, 0),
            secret_room: Rect::new(0, 0, 0, 0),
        }
    }
}
