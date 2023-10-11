pub struct Device;

impl Device {
    pub fn parse(input: &str) -> (u8, u8, Operation, Formatting) {
        let (left, rest) = input.split_once(';').unwrap();
        let (right, rest) = rest.split_once(';').unwrap();
        let (operation, formatting) = rest.split_once(';').unwrap();

        let left = left.replace('X', "0").replace('Y', "1");
        let right = right.replace('X', "0").replace('Y', "1");

        let left = u8::from_str_radix(left.trim(), 2).unwrap();
        let right = u8::from_str_radix(right.trim(), 2).unwrap();

        let operation = match operation.trim() {
            "G" => Operation::Add,
            "L" => Operation::Sub,
            "W" => Operation::Mul,
            op => unimplemented!("'{}' operation not implemented", op),
        };

        let formatting = match formatting.trim() {
            "Q" => Formatting::Binary,
            "E" => Formatting::Decimal,
            format => unimplemented!("'{}' formatting type not implemented", format),
        };

        (left, right, operation, formatting)
    }

    pub fn run(left: u8, right: u8, op: Operation) -> u8 {
        match op {
            Operation::Add => left.wrapping_add(right),
            Operation::Sub => left.wrapping_sub(right),
            Operation::Mul => left.wrapping_mul(right),
        }
    }

    pub fn format(result: u8, formatting: Formatting) -> String {
        match formatting {
            Formatting::Binary => format!("{:b}", result),
            Formatting::Decimal => format!("{}", result),
        }
    }
}

pub enum Operation {
    Add,
    Sub,
    Mul,
}

pub enum Formatting {
    Binary,
    Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device() {
        let tests: Vec<(&'static str, &'static str)> = vec![
            ("XXXXXXXX; XXXXXXXX; G; Q", "0"),
            ("XXXXXXXY; XXXXXXXX; G; Q", "1"),
            ("XXXXXXXY; XXXXXXXY; G; Q", "10"),
            ("YYXXXXXX; YYYXXXXY; G; Q", "10100001"),
            ("XXXXXXXX; XXXXXXXY; L; Q", "11111111"),
            ("XXXXYYXY; XXXXYYYY; L; Q", "11111110"),
            ("XXXXXXXY; XXXXXXXY; W; Q", "1"),
            ("YYXXYXXY; XXXYXYYX; W; Q", "1000110"),
            ("YXXXYXYY; XYYXXYXX; W; E", "76"),
            ("XYYYXXXX; YYYYXXXX; L; Q", "10000000"),
            ("XXYXXYXY; XYXXYXYX; W; Q", "10110010"),
            ("YYYXXXXY; YYXXYXYX; G; E", "171"),
        ];

        for test in tests {
            assert_eq!(run_device(test.0), test.1)
        }
    }

    fn run_device(input: &str) -> String {
        let (left, right, operation, formatting) = Device::parse(input);
        let result = Device::run(left, right, operation);
        Device::format(result, formatting)
    }
}
