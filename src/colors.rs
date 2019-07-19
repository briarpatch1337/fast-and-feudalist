
use drawing::ColorSpec;
use gameboard::GameBoardSpaceType;
use PlayerColor;

// This is like defining an interface.  We'll have enums implement this trait if there are colors associated with the enum value.
pub trait Color {
    fn color(&self) -> ColorSpec;
}

impl Color for GameBoardSpaceType
{
    fn color(&self) -> ColorSpec {
        match self {
            GameBoardSpaceType::Void => ColorSpec {
                r: 0x00,
                g: 0x00,
                b: 0x00
            },
            GameBoardSpaceType::Water => ColorSpec {
                r: 0x20,
                g: 0x20,
                b: 0x80
            },
            GameBoardSpaceType::Mountain => ColorSpec {
                r: 0x40,
                g: 0x40,
                b: 0x40
            },
            GameBoardSpaceType::Forest => ColorSpec {
                r: 0x11,
                g: 0x46,
                b: 0x11,
            },
            GameBoardSpaceType::Plains => ColorSpec {
                r: 0x00,
                g: 0x80,
                b: 0x40
            },
            GameBoardSpaceType::Field => ColorSpec {
                r: 0x80,
                g: 0x70,
                b: 0x00
            }
        }
    }
}

impl Color for PlayerColor {
    fn color(&self) -> ColorSpec {
        match self {
            PlayerColor::Red => ColorSpec {
                r: 0xC0,
                g: 0x00,
                b: 0x00
            },
            PlayerColor::Blue => ColorSpec {
                r: 0x00,
                g: 0x00,
                b: 0xFF
            },
            PlayerColor::Green => ColorSpec {
                r: 0x00,
                g: 0xD0,
                b: 0x00
            },
            PlayerColor::Yellow => ColorSpec {
                r: 0xFF,
                g: 0xD7,
                b: 0x00,
            }
        }
    }
}
