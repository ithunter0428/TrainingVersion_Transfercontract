//#![cfg(test)]
#![cfg_attr(feature = "custom_attribute", feature(custom_attribute))]
#![allow(dead_code)]
#![allow(unused_attributes)]

extern crate reflection;
#[macro_use]
extern crate reflection_derive;
extern crate trees;

#[cfg(test)]
use reflection::Member;
use reflection::Reflection;
#[cfg(test)]
use trees::Node;

#[derive(Reflection)]
struct S0;

#[derive(Reflection)]
struct SS0(S0);

#[derive(Reflection)]
struct S1<'a> {
    bool_: bool,
    i8_: i8,
    u8_: u8,
    i16_: i16,
    u16_: u16,
    i32_: i32,
    u32_: u32,
    i64_: i64,
    u64_: u64,
    f32_: f32,
    f64_: f64,
    str_: &'a str,
    string: String,
}

#[derive(Reflection)]
struct St1<'a>(
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64,
    &'a str,
    String,
);

#[derive(Reflection)]
struct SS1<'a>(S1<'a>);

#[derive(Reflection)]
struct SSt1<'a>(St1<'a>);

#[derive(Reflection)]
struct SString(String);

#[derive(Reflection)]
enum EEE {
    UUU((((),),)),
}

#[derive(Reflection)]
struct Color(
    u32, // red
    u32, // green
    u32, // blue
    #[cfg(feature = "custom_attribute")]
    #[serde(skip_serializing)]
    u32, // alpha
);

#[derive(Reflection)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
    #[cfg(feature = "custom_attribute")]
    #[serde(skip_serializing)]
    t: u32,
}

#[derive(Reflection)]
struct ColoredPoint {
    point: Point,
    #[cfg(feature = "custom_attribute")]
    #[serde(rename = "colour")]
    color: Color,
    #[cfg(not(feature = "custom_attribute"))]
    colour: Color,
}

#[derive(Reflection)]
struct StColoredPoint(Point, Color);

#[derive(Reflection)]
enum TrippleU32s {
    ColorT(Color),
    ColorS {
        color: Color,
    },
    FlatColor(
        u32,
        u32,
        u32,
        #[cfg(feature = "custom_attribute")]
        #[serde(skip_serializing)]
        u32,
    ),
    PointT(Point),
    PointS {
        point: Point,
    },
    FlatPoint(
        #[cfg(feature = "custom_attribute")]
        #[serde(skip_serializing)]
        u32,
        #[cfg(feature = "custom_attribute")]
        #[serde(skip_serializing)]
        u32,
        #[cfg(feature = "custom_attribute")]
        #[serde(skip_serializing)]
        u32,
    ),
}

#[derive(Reflection)]
struct ArraysWithLength {
    arr1: [u8; 1],
    arr2: [u8; 2],
    arr3: [u8; 3],
    arr4: [u8; 4],
    arr5: [u8; 5],
    arr6: [u8; 6],
    arr7: [u8; 7],
    arr8: [u8; 8],
    arr9: [u8; 9],
    arr10: [u8; 10],
    arr11: [u8; 11],
    arr12: [u8; 12],
    arr13: [u8; 13],
    arr14: [u8; 14],
    arr15: [u8; 15],
    arr16: [u8; 16],
    arr17: [u8; 17],
    arr18: [u8; 18],
    arr19: [u8; 19],
    arr21: [u8; 21],
    arr22: [u8; 22],
    arr23: [u8; 23],
    arr24: [u8; 24],
    arr25: [u8; 25],
    arr26: [u8; 26],
    arr27: [u8; 27],
    arr28: [u8; 28],
    arr29: [u8; 29],
    arr30: [u8; 30],
    arr31: [u8; 31],
    arr32: [u8; 32],
    arr33: [u8; 33],
    arr34: [u8; 34],
    arr35: [u8; 35],
    arr36: [u8; 36],
    arr37: [u8; 37],
    arr38: [u8; 38],
    arr39: [u8; 39],
    arr40: [u8; 40],
    arr41: [u8; 41],
    arr42: [u8; 42],
    arr43: [u8; 43],
    arr44: [u8; 44],
    arr45: [u8; 45],
    arr46: [u8; 46],
    arr47: [u8; 47],
    arr48: [u8; 48],
    arr49: [u8; 49],
    arr50: [u8; 50],
    arr51: [u8; 51],
    arr52: [u8; 52],
    arr53: [u8; 53],
    arr54: [u8; 54],
    arr55: [u8; 55],
    arr56: [u8; 56],
    arr57: [u8; 57],
    arr58: [u8; 58],
    arr59: [u8; 59],
    arr60: [u8; 60],
    arr61: [u8; 61],
    arr62: [u8; 62],
    arr63: [u8; 63],
    arr64: [u8; 64],
}

#[test]
fn arrays_with_length() {
    assert_eq!(ArraysWithLength::schemata().to_string(), "_:ArraysWithLength( arr1:[u8; 1]( _:u8 ) arr2:[u8; 2]( _:u8 ) arr3:[u8; 3]( _:u8 ) arr4:[u8; 4]( _:u8 ) arr5:[u8; 5]( _:u8 ) arr6:[u8; 6]( _:u8 ) arr7:[u8; 7]( _:u8 ) arr8:[u8; 8]( _:u8 ) arr9:[u8; 9]( _:u8 ) arr10:[u8; 10]( _:u8 ) arr11:[u8; 11]( _:u8 ) arr12:[u8; 12]( _:u8 ) arr13:[u8; 13]( _:u8 ) arr14:[u8; 14]( _:u8 ) arr15:[u8; 15]( _:u8 ) arr16:[u8; 16]( _:u8 ) arr17:[u8; 17]( _:u8 ) arr18:[u8; 18]( _:u8 ) arr19:[u8; 19]( _:u8 ) arr21:[u8; 21]( _:u8 ) arr22:[u8; 22]( _:u8 ) arr23:[u8; 23]( _:u8 ) arr24:[u8; 24]( _:u8 ) arr25:[u8; 25]( _:u8 ) arr26:[u8; 26]( _:u8 ) arr27:[u8; 27]( _:u8 ) arr28:[u8; 28]( _:u8 ) arr29:[u8; 29]( _:u8 ) arr30:[u8; 30]( _:u8 ) arr31:[u8; 31]( _:u8 ) arr32:[u8; 32]( _:u8 ) arr33:[u8; 33]( _:u8 ) arr34:[u8; 34]( _:u8 ) arr35:[u8; 35]( _:u8 ) arr36:[u8; 36]( _:u8 ) arr37:[u8; 37]( _:u8 ) arr38:[u8; 38]( _:u8 ) arr39:[u8; 39]( _:u8 ) arr40:[u8; 40]( _:u8 ) arr41:[u8; 41]( _:u8 ) arr42:[u8; 42]( _:u8 ) arr43:[u8; 43]( _:u8 ) arr44:[u8; 44]( _:u8 ) arr45:[u8; 45]( _:u8 ) arr46:[u8; 46]( _:u8 ) arr47:[u8; 47]( _:u8 ) arr48:[u8; 48]( _:u8 ) arr49:[u8; 49]( _:u8 ) arr50:[u8; 50]( _:u8 ) arr51:[u8; 51]( _:u8 ) arr52:[u8; 52]( _:u8 ) arr53:[u8; 53]( _:u8 ) arr54:[u8; 54]( _:u8 ) arr55:[u8; 55]( _:u8 ) arr56:[u8; 56]( _:u8 ) arr57:[u8; 57]( _:u8 ) arr58:[u8; 58]( _:u8 ) arr59:[u8; 59]( _:u8 ) arr60:[u8; 60]( _:u8 ) arr61:[u8; 61]( _:u8 ) arr62:[u8; 62]( _:u8 ) arr63:[u8; 63]( _:u8 ) arr64:[u8; 64]( _:u8 ) )");
}

#[test]
fn misc() {
    assert_eq!(S0::schemata().to_string(), "_:S0");
    assert_eq!(SS0::schemata().to_string(), "_:SS0( 0:S0 )");
    assert_eq!(S1::schemata().to_string(), "_:S1( bool_:bool i8_:i8 u8_:u8 i16_:i16 u16_:u16 i32_:i32 u32_:u32 i64_:i64 u64_:u64 f32_:f32 f64_:f64 str_:&str string:String )");
    assert_eq!(St1::schemata().to_string(), "_:St1( 0:bool 1:i8 2:u8 3:i16 4:u16 5:i32 6:u32 7:i64 8:u64 9:f32 10:f64 11:&str 12:String )");
    assert_eq!(SString::schemata().to_string(), "_:SString( 0:String )");
    assert_eq!(
        EEE::schemata().to_string(),
        "_:EEE( UUU|( 0:(((),),)( 0:((),)( 0:() ) ) ) )"
    );
    assert_eq!(
        Color::schemata().to_string(),
        "_:Color( 0:u32 1:u32 2:u32 )"
    );
    assert_eq!(
        Point::schemata().to_string(),
        "_:Point( x:u32 y:u32 z:u32 )"
    );
    assert_eq!(
        <[Point]>::schemata().to_string(),
        "_:[Point]( _:Point( x:u32 y:u32 z:u32 ) )"
    );
    assert_eq!(
        Vec::<Point>::schemata().to_string(),
        "_:Vec<Point>( _:Point( x:u32 y:u32 z:u32 ) )"
    );
    assert_eq!(
        ColoredPoint::schemata().to_string(),
        "_:ColoredPoint( point:Point( x:u32 y:u32 z:u32 ) colour:Color( 0:u32 1:u32 2:u32 ) )"
    );
    assert_eq!(
        StColoredPoint::schemata().to_string(),
        "_:StColoredPoint( 0:Point( x:u32 y:u32 z:u32 ) 1:Color( 0:u32 1:u32 2:u32 ) )"
    );
    assert_eq!(
        <(Color, Point)>::schemata().to_string(),
        "_:(Color,Point)( 0:Color( 0:u32 1:u32 2:u32 ) 1:Point( x:u32 y:u32 z:u32 ) )"
    );
    assert_eq!(TrippleU32s::schemata().to_string(),
               "_:TrippleU32s( ColorT|( 0:Color( 0:u32 1:u32 2:u32 ) ) ColorS|( color:Color( 0:u32 1:u32 2:u32 ) ) FlatColor|( 0:u32 1:u32 2:u32 ) \
           PointT|( 0:Point( x:u32 y:u32 z:u32 ) ) PointS|( point:Point( x:u32 y:u32 z:u32 ) ) FlatPoint| )"
    );
}

#[test]
fn pointer() {
    #[derive(Reflection)]
    struct Link<'a, T: 'a + Reflection> {
        data: T,
        cptr: *const Link<'a, T>,
        ptr: *mut Link<'a, T>,
        nn: std::ptr::NonNull<Link<'a, T>>,
        cref: &'a Link<'a, T>,
        mref: &'a mut Link<'a, T>,
        boxed: Box<Link<'a, T>>,
        rc: std::rc::Rc<Link<'a, T>>,
    }
    assert_eq!(
        Link::<i32>::schemata().to_string(),
        "_:Link( \
            data:i32 \
            cptr:*const Link \
            ptr:*mut Link \
            nn:NonNull<Link> \
            cref:&Link \
            mref:&mut Link \
            boxed:Box<Link> \
            rc:Rc<Link> )"
    );
}

#[test]
// see https://github.com/serde-rs/serde/issues/345
fn serde_issue_345() {
    #[derive(Reflection)]
    enum Foo {
        Bar { a: u64 },
        Bla { b: u64 },
    }

    fn schema_to_string(node: &Node<Member>, nth: usize, level: usize) -> String {
        match node.data() {
            Member::Field(ref field) => {
                if field.ty == reflection::Type::Enum {
                    format!(
                        "{0}type: {1:?},\n{0}name: {2:?},\n{0}cases: {{\n{3}{0}}}",
                        " ".repeat(level * 4),
                        &field.tyname.clone().unwrap_or_default(),
                        field.id,
                        members_to_string(node, level)
                    )
                } else {
                    format!(
                        "{0}type: {1:?},\n{0}name: {2:?},{3}",
                        " ".repeat(level * 4),
                        &field.tyname.clone().unwrap_or_default(),
                        field.id,
                        members_to_string(node, level)
                    )
                }
            }
            Member::Variant(ref variant) => format!(
                "{0}{1} => {{\n    {0}type: \"enum_val\",\n    {0}name: {2:?},{3}{0}}}",
                " ".repeat(level * 4),
                nth,
                variant.id,
                members_to_string(node, level + 1)
            ),
        }
    }

    fn members_to_string(node: &Node<Member>, level: usize) -> String {
        let mut s = String::new();
        let mut nth = 0usize;
        for child in node.iter() {
            s.push_str(&(schema_to_string(child, nth, level + 1) + &"\n"));
            nth += 1;
        }
        if nth == 0 {
            String::new()
        } else {
            if let Member::Field(ref field) = node.data() {
                if field.ty == reflection::Type::Enum {
                    return s;
                }
            }
            format!("\n{0}fields: [\n{1}{0}]\n", " ".repeat(level * 4), s)
        }
    }

    assert_eq!(
        schema_to_string(Foo::schemata().root(), 0, 0),
        r#"type: "Foo",
name: "_",
cases: {
    0 => {
        type: "enum_val",
        name: "Bar",
        fields: [
            type: "u64",
            name: "a",
        ]
    }
    1 => {
        type: "enum_val",
        name: "Bla",
        fields: [
            type: "u64",
            name: "b",
        ]
    }
}"#
    );
}
