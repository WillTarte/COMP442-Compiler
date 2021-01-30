const VALID_CHARS: &str = "=<>+-*/|&!?(){}[];,.:";

pub(crate) fn is_valid_character(c: char) -> bool
{
    for vc in VALID_CHARS.chars()
    {
        if c == vc
        {
            return true;
        }

    }
    return false;
}