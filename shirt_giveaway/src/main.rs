#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, userPref: Option<ShirtColor>) -> ShirtColor {
        userPref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut n_red = 0;
        let mut n_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => n_red += 1,
                ShirtColor::Blue => n_blue += 1,
            }
        }

        if n_blue > n_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let userPref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(userPref1);

    println!(
        "The user with preference {:?} gets {:?}",
        userPref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
