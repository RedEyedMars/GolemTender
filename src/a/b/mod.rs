use crate::a::GameState;
use crate::g::animation::animaton::{Animation, SizeMode};
use crate::g::animation::img::Img;
use crate::g::resources::Resources;

use crate::a::e::gol::GolemState;
use crate::a::*;
use packed_simd::u32x2;

pub mod blocks;
use blocks::*;
pub mod tiles;
use tiles::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Wall {
    BlockWall,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Board {
    tiles: Vec<Tile>,
    pub name: String,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(
        name: String,
        width: usize,
        height: usize,
        res: &mut Resources,
    ) -> Result<Board, failure::Error> {
        let mut tiles = Vec::with_capacity(width * height);
        for _ in 0..height {
            for _ in 0..width {
                tiles.push(Tile::Empty);
            }
        }
        Ok(Board {
            /*
            grid_img: Img::new(
                "bot_1.png".to_string(),
                0f32,
                SizeMode::Bot,
                Animation::MainXShift2x16,
                res,
            )?,*/
            tiles: tiles,
            name: name,
            width: width,
            height: height,
        })
    }
    pub fn execute(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }
    pub fn render(&self, game: &GameState) -> Result<(), failure::Error> {
        //self.grid_img.render(game);
        Ok(())
    }
    pub fn render_text(&self) {
        for y in 0..self.height as u32 {
            for x in 0..self.width as u32 {
                self.tiles
                    .get(self.index(u32x2::new(x, (self.height - 1) as u32 - y)))
                    .unwrap()
                    .render_text();
            }
            println!("");
        }
        println!("===================================================");
    }
    pub fn walls(&self) -> Vec<(&Wall, u32x2)> {
        let mut result = Vec::new();
        for (index, tile) in self.tiles.iter().enumerate() {
            if let Tile::Occupied(Occupant::Wall(wall)) = tile {
                result.push((wall, self.xy(index)));
            }
        }
        result
    }
    pub fn blocks(&self) -> Vec<(&Block, u32x2)> {
        let mut result = Vec::new();
        for (index, tile) in self.tiles.iter().enumerate() {
            if let Tile::Occupied(Occupant::Block(block)) = tile {
                result.push((block, self.xy(index)));
            }
        }
        result
    }

    pub fn add_golem(&mut self, (xy, gol): (u32x2, GolemState)) -> Result<(), ()> {
        self.apply_on_tile(&gol, xy, &Tile::add_golem)?;
        Ok(())
    }
    pub fn add_block(&mut self, (xy, block): (u32x2, Block)) -> Result<(), ()> {
        self.apply_on_tile(&block, xy, &Tile::add_block)?;
        Ok(())
    }
    pub fn add_occupant(&mut self, (xy, occupant): (u32x2, Occupant)) -> Result<(), ()> {
        self.apply_on_tile(&occupant, xy, &Tile::add_occupant_ref)?;
        Ok(())
    }

    pub fn md(&mut self, _x: f32, _y: f32) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn mu(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn get_golem(&self, xy: u32x2) -> Result<GolemState, ()> {
        Ok(self.tiles.get(self.index(xy)).unwrap().get_golem()?)
    }

    pub fn index(&self, xy: u32x2) -> usize {
        (xy.extract(1) as usize) * self.width + (xy.extract(0) as usize)
    }
    pub fn xy(&self, i: usize) -> u32x2 {
        u32x2::new(i as u32 % self.width as u32, i as u32 / self.width as u32)
    }

    pub fn grab(&mut self, new_state: bool, gol_xy: u32x2) -> Result<GolemState, ()> {
        Ok(self
            .apply_on_tile(&new_state, gol_xy, &Tile::grab)?
            .get_golem()?)
    }
    pub fn lift(&mut self, gol_xy: u32x2) -> Result<GolemState, ()> {
        let golem = self.tiles.get(self.index(gol_xy)).unwrap().get_golem()?;
        let target_xy = golem.advance(gol_xy, self.width as u32, self.height as u32)?;
        let target_block = {
            let index =
                (target_xy.extract(1) as usize) * self.width + (target_xy.extract(0) as usize);
            let tile = self.tiles.get(index).unwrap();
            match tile {
                Tile::Occupied(Occupant::Block(b)) => Ok(b.clone()),
                _ => Err(()),
            }?
        };
        if let (Tile::Occupied(Occupant::Golem(new_golem)), _) = self.apply_on_both_tile(
            &target_block,
            gol_xy,
            &Tile::lift_onto_golem,
            target_xy,
            &Tile::lift_target,
        )? {
            Ok(new_golem)
        } else {
            Err(())
        }
    }
    pub fn turn(
        &mut self,
        gol_xy: u32x2,
        t: &dyn Fn(Direction) -> Direction,
    ) -> Result<GolemState, ()> {
        use std::mem::replace;
        let index1 = self.index(gol_xy);
        let golem = self.tiles.get(index1).unwrap().get_golem()?;
        if golem.grabbing() {
            if let Ok((from_xy, to_xy)) =
                golem.advance_and_rotated(gol_xy, self.width as u32, self.height as u32, t)
            {
                let index_to = self.index(to_xy);
                let tile_to = self.tiles.get(index_to).unwrap().clone();
                if tile_to.is_acceptable() {
                    let index_from = self.index(from_xy);
                    let tile_from = self.tiles.get(index_from).unwrap().clone();
                    if let Some(pushable) = tile_from.pushable_occupant() {
                        let _ = replace(&mut self.tiles[index_from], tile_from.remove_occupant()?);
                        let _ = replace(&mut self.tiles[index_to], tile_to.add_occupant(pushable)?);
                    }
                }
            }
        }
        let _ = replace(
            &mut self.tiles[index1],
            Tile::Occupied(Occupant::Golem(golem.from_direction(t))),
        );
        Ok(golem.from_direction(t))
    }

    pub fn advance(&mut self, gol_xy: u32x2) -> Result<u32x2, ()> {
        use std::mem::replace;
        let index1 = self.index(gol_xy);
        let golem_tile = self.tiles.get(index1).unwrap().clone();
        let golem = golem_tile.get_golem()?;
        let tile_xy = golem.advance(gol_xy, self.width as u32, self.height as u32)?;
        let index2 = self.index(tile_xy);
        let tile = self.tiles.get(index2).unwrap().clone();
        if tile.is_steppable() {
            if let Some(pushable) = tile.pushable_occupant() {
                let index3 =
                    self.index(golem.advance(tile_xy, self.width as u32, self.height as u32)?);
                let tile2 = self.tiles.get(index3).unwrap().clone();
                if !tile2.is_steppable() {
                    return Err(());
                }
                let _ = replace(&mut self.tiles[index3], tile2.add_occupant(pushable)?);
            }
            let _ = replace(
                &mut self.tiles[index2],
                tile.remove_occupant()?
                    .add_occupant(Occupant::Golem(golem))?,
            );
            let _ = replace(&mut self.tiles[index1], golem_tile.remove_occupant()?);
            Ok(tile_xy)
        } else {
            Err(())
        }
    }

    pub fn retreat(&mut self, gol_xy: u32x2) -> Result<(u32x2, GolemState), ()> {
        use std::mem::replace;
        let index1 = self.index(gol_xy);
        let golem_tile = self.tiles.get(index1).unwrap().clone();
        let golem = golem_tile.get_golem()?;
        let tile_xy = golem.retreat(gol_xy, self.width as u32, self.height as u32)?;
        let index2 = self.index(tile_xy);
        let tile = self.tiles.get(index2).unwrap().clone();
        if tile.is_steppable() {
            if let Some(pushable) = tile.pushable_occupant() {
                if let Ok(tile_xy2) = golem.retreat(tile_xy, self.width as u32, self.height as u32)
                {
                    let index3 = self.index(tile_xy2);
                    let tile2 = self.tiles.get(index3).unwrap();
                    if tile2.is_steppable() {
                        let _ = replace(&mut self.tiles[index3], tile.add_occupant(pushable)?);
                    } else {
                        return Err(());
                    }
                }
            }
            let _ = replace(
                &mut self.tiles[index2],
                tile.add_occupant(Occupant::Golem(golem.clone()))?,
            );
            let _ = replace(&mut self.tiles[index1], golem_tile.remove_occupant()?);
        } else {
            return Err(());
        }
        if golem.grabbing() {
            if let Ok(tile_xy2) = golem.advance(gol_xy, self.width as u32, self.height as u32) {
                let index3 = self.index(tile_xy2);
                let tile2 = self.tiles.get(index3).unwrap().clone();
                if let Some(pushable) = tile.pushable_occupant() {
                    let _ = replace(&mut self.tiles[index3], tile2.remove_occupant()?);
                    let _ = replace(&mut self.tiles[index1], golem_tile.add_occupant(pushable)?);
                }
            }
        }
        Ok((tile_xy, golem))
    }

    pub fn apply_on_tile<T>(
        &mut self,
        obj: &T,
        xy: u32x2,
        f: &dyn Fn(&Tile, &T) -> Result<Tile, ()>,
    ) -> Result<Tile, ()>
    where
        T: std::any::Any,
    {
        use std::mem::replace;
        let index = (xy.extract(1) as usize) * self.width + (xy.extract(0) as usize);
        let tile = self.tiles.get(index).unwrap().clone();
        let v = f(&tile, obj)?;
        let _ = replace(&mut self.tiles[index], v.clone());
        Ok(v)
    }

    pub fn apply_on_both_tile<T>(
        &mut self,
        obj: &T,
        xy1: u32x2,
        f1: &dyn Fn(&Tile, &T) -> Result<Tile, ()>,
        xy2: u32x2,
        f2: &dyn Fn(&Tile, &T) -> Result<Tile, ()>,
    ) -> Result<(Tile, Tile), ()>
    where
        T: std::any::Any,
    {
        use std::mem::replace;
        let index1 = (xy1.extract(1) as usize) * self.width + (xy1.extract(0) as usize);
        let index2 = (xy2.extract(1) as usize) * self.width + (xy2.extract(0) as usize);
        let tile1 = self.tiles.get(index1).unwrap().clone();
        let tile2 = self.tiles.get(index2).unwrap().clone();
        let v = (f1(&tile1, obj)?, f2(&tile2, obj)?);
        let _ = replace(&mut self.tiles[index1], v.0.clone());
        let _ = replace(&mut self.tiles[index2], v.1.clone());
        Ok(v)
    }
}
