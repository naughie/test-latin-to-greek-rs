use crate::unicode as uc;
use crate::{Accent, Breathing, Case, Diaeresis, Subscript};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Initial,
    Alpha(Case, Breathing, Accent, Subscript),
    Beta(Case),
    Gamma(Case),
    Delta(Case),
    Epsilon(Case, Breathing, Accent),
    Zeta(Case),
    Eta(Case, Breathing, Accent, Subscript),
    Theta(Case),
    Iota(Case, Breathing, Accent, Diaeresis),
    Kappa(Case),
    Lambda(Case),
    Mu(Case),
    Nu(Case),
    Xi(Case),
    Omicron(Case, Breathing, Accent),
    Pi(Case),
    Rho(Case, Breathing),
    Sigma(Case),
    Tau(Case),
    Ypsilon(Case, Breathing, Accent, Diaeresis),
    Phi(Case),
    Chi(Case),
    Psi(Case),
    Omega(Case, Breathing, Accent, Subscript),
}

fn transit(state: State, c: u8) -> State {
    use State::*;

    match c {
        b'A' => Alpha(
            Case::Capital,
            Breathing::None,
            Accent::None,
            Subscript::None,
        ),
        b'B' => Beta(Case::Capital),
        b'G' => Gamma(Case::Capital),
        b'D' => Delta(Case::Capital),
        b'E' => Epsilon(Case::Capital, Breathing::None, Accent::None),
        b'Z' => Zeta(Case::Capital),
        b'H' => Eta(
            Case::Capital,
            Breathing::None,
            Accent::None,
            Subscript::None,
        ),
        b'Q' => Theta(Case::Capital),
        b'I' => Iota(
            Case::Capital,
            Breathing::None,
            Accent::None,
            Diaeresis::None,
        ),
        b'K' => Kappa(Case::Capital),
        b'L' => Lambda(Case::Capital),
        b'M' => Mu(Case::Capital),
        b'N' => Nu(Case::Capital),
        b'X' => Xi(Case::Capital),
        b'O' => Omicron(Case::Capital, Breathing::None, Accent::None),
        b'P' => Pi(Case::Capital),
        b'R' => Rho(Case::Capital, Breathing::None),
        b'S' => Sigma(Case::Capital),
        b'T' => Tau(Case::Capital),
        b'Y' => Ypsilon(
            Case::Capital,
            Breathing::None,
            Accent::None,
            Diaeresis::None,
        ),
        b'F' => Phi(Case::Capital),
        b'C' => Chi(Case::Capital),
        b'J' => Psi(Case::Capital),
        b'W' => Omega(
            Case::Capital,
            Breathing::None,
            Accent::None,
            Subscript::None,
        ),

        b'a' => Alpha(Case::Small, Breathing::None, Accent::None, Subscript::None),
        b'b' => Beta(Case::Small),
        b'g' => Gamma(Case::Small),
        b'd' => Delta(Case::Small),
        b'e' => Epsilon(Case::Small, Breathing::None, Accent::None),
        b'z' => Zeta(Case::Small),
        b'h' => Eta(Case::Small, Breathing::None, Accent::None, Subscript::None),
        b'q' => Theta(Case::Small),
        b'i' => Iota(Case::Small, Breathing::None, Accent::None, Diaeresis::None),
        b'k' => Kappa(Case::Small),
        b'l' => Lambda(Case::Small),
        b'm' => Mu(Case::Small),
        b'n' => Nu(Case::Small),
        b'x' => Xi(Case::Small),
        b'o' => Omicron(Case::Small, Breathing::None, Accent::None),
        b'p' => Pi(Case::Small),
        b'r' => Rho(Case::Small, Breathing::None),
        b's' => Sigma(Case::Small),
        b't' => Tau(Case::Small),
        b'y' => Ypsilon(Case::Small, Breathing::None, Accent::None, Diaeresis::None),
        b'f' => Phi(Case::Small),
        b'c' => Chi(Case::Small),
        b'j' => Psi(Case::Small),
        b'w' => Omega(Case::Small, Breathing::None, Accent::None, Subscript::None),

        b')' => toggle_smooth(state),
        b'(' => toggle_rough(state),
        b'\\' => toggle_grave(state),
        b'/' => toggle_acute(state),
        b'=' => toggle_circumflex(state),
        b'|' => toggle_iota(state),
        b'"' => toggle_diaeresis(state),

        _ => Initial,
    }
}

fn is_diacritics(c: u8) -> bool {
    matches!(c, b')' | b'(' | b'\\' | b'/' | b'=' | b'|' | b'"')
}

fn toggle_smooth(state: State) -> State {
    use State::*;

    match state {
        Alpha(a, Breathing::Smooth, c, d) => Alpha(a, Breathing::None, c, d),
        Alpha(a, _, c, d) => Alpha(a, Breathing::Smooth, c, d),
        Epsilon(a, Breathing::Smooth, c) => Epsilon(a, Breathing::None, c),
        Epsilon(a, _, c) => Epsilon(a, Breathing::Smooth, c),
        Eta(a, Breathing::Smooth, c, d) => Eta(a, Breathing::None, c, d),
        Eta(a, _, c, d) => Eta(a, Breathing::Smooth, c, d),
        Iota(a, Breathing::Smooth, c, d) => Iota(a, Breathing::None, c, d),
        Iota(a, _, c, d) => Iota(a, Breathing::Smooth, c, d),
        Omicron(a, Breathing::Smooth, c) => Omicron(a, Breathing::None, c),
        Omicron(a, _, c) => Omicron(a, Breathing::Smooth, c),
        Ypsilon(a, Breathing::Smooth, c, d) => Ypsilon(a, Breathing::None, c, d),
        Ypsilon(a, _, c, d) => Ypsilon(a, Breathing::Smooth, c, d),
        Omega(a, Breathing::Smooth, c, d) => Omega(a, Breathing::None, c, d),
        Omega(a, _, c, d) => Omega(a, Breathing::Smooth, c, d),
        _ => state,
    }
}
fn toggle_rough(state: State) -> State {
    use State::*;

    match state {
        Alpha(a, Breathing::Rough, c, d) => Alpha(a, Breathing::None, c, d),
        Alpha(a, _, c, d) => Alpha(a, Breathing::Rough, c, d),
        Epsilon(a, Breathing::Rough, c) => Epsilon(a, Breathing::None, c),
        Epsilon(a, _, c) => Epsilon(a, Breathing::Rough, c),
        Eta(a, Breathing::Rough, c, d) => Eta(a, Breathing::None, c, d),
        Eta(a, _, c, d) => Eta(a, Breathing::Rough, c, d),
        Iota(a, Breathing::Rough, c, d) => Iota(a, Breathing::None, c, d),
        Iota(a, _, c, d) => Iota(a, Breathing::Rough, c, d),
        Omicron(a, Breathing::Rough, c) => Omicron(a, Breathing::None, c),
        Omicron(a, _, c) => Omicron(a, Breathing::Rough, c),
        Ypsilon(a, Breathing::Rough, c, d) => Ypsilon(a, Breathing::None, c, d),
        Ypsilon(a, _, c, d) => Ypsilon(a, Breathing::Rough, c, d),
        Omega(a, Breathing::Rough, c, d) => Omega(a, Breathing::None, c, d),
        Omega(a, _, c, d) => Omega(a, Breathing::Rough, c, d),
        _ => state,
    }
}

fn toggle_grave(state: State) -> State {
    use State::*;

    match state {
        Alpha(a, b, Accent::Grave, d) => Alpha(a, b, Accent::None, d),
        Alpha(a, b, _, d) => Alpha(a, b, Accent::Grave, d),
        Epsilon(a, b, Accent::Grave) => Epsilon(a, b, Accent::None),
        Epsilon(a, b, _) => Epsilon(a, b, Accent::Grave),
        Eta(a, b, Accent::Grave, d) => Eta(a, b, Accent::None, d),
        Eta(a, b, _, d) => Eta(a, b, Accent::Grave, d),
        Iota(a, b, Accent::Grave, d) => Iota(a, b, Accent::None, d),
        Iota(a, b, _, d) => Iota(a, b, Accent::Grave, d),
        Omicron(a, b, Accent::Grave) => Omicron(a, b, Accent::None),
        Omicron(a, b, _) => Omicron(a, b, Accent::Grave),
        Ypsilon(a, b, Accent::Grave, d) => Ypsilon(a, b, Accent::None, d),
        Ypsilon(a, b, _, d) => Ypsilon(a, b, Accent::Grave, d),
        Omega(a, b, Accent::Grave, d) => Omega(a, b, Accent::None, d),
        Omega(a, b, _, d) => Omega(a, b, Accent::Grave, d),
        _ => state,
    }
}
fn toggle_acute(state: State) -> State {
    use State::*;

    match state {
        Alpha(a, b, Accent::Acute, d) => Alpha(a, b, Accent::None, d),
        Alpha(a, b, _, d) => Alpha(a, b, Accent::Acute, d),
        Epsilon(a, b, Accent::Acute) => Epsilon(a, b, Accent::None),
        Epsilon(a, b, _) => Epsilon(a, b, Accent::Acute),
        Eta(a, b, Accent::Acute, d) => Eta(a, b, Accent::None, d),
        Eta(a, b, _, d) => Eta(a, b, Accent::Acute, d),
        Iota(a, b, Accent::Acute, d) => Iota(a, b, Accent::None, d),
        Iota(a, b, _, d) => Iota(a, b, Accent::Acute, d),
        Omicron(a, b, Accent::Acute) => Omicron(a, b, Accent::None),
        Omicron(a, b, _) => Omicron(a, b, Accent::Acute),
        Ypsilon(a, b, Accent::Acute, d) => Ypsilon(a, b, Accent::None, d),
        Ypsilon(a, b, _, d) => Ypsilon(a, b, Accent::Acute, d),
        Omega(a, b, Accent::Acute, d) => Omega(a, b, Accent::None, d),
        Omega(a, b, _, d) => Omega(a, b, Accent::Acute, d),
        _ => state,
    }
}
fn toggle_circumflex(state: State) -> State {
    use State::*;

    match state {
        Alpha(a, b, Accent::Circumflex, d) => Alpha(a, b, Accent::None, d),
        Alpha(a, b, _, d) => Alpha(a, b, Accent::Circumflex, d),
        Eta(a, b, Accent::Circumflex, d) => Eta(a, b, Accent::None, d),
        Eta(a, b, _, d) => Eta(a, b, Accent::Circumflex, d),
        Iota(a, b, Accent::Circumflex, d) => Iota(a, b, Accent::None, d),
        Iota(a, b, _, d) => Iota(a, b, Accent::Circumflex, d),
        Ypsilon(a, b, Accent::Circumflex, d) => Ypsilon(a, b, Accent::None, d),
        Ypsilon(a, b, _, d) => Ypsilon(a, b, Accent::Circumflex, d),
        Omega(a, b, Accent::Circumflex, d) => Omega(a, b, Accent::None, d),
        Omega(a, b, _, d) => Omega(a, b, Accent::Circumflex, d),
        _ => state,
    }
}

fn toggle_iota(state: State) -> State {
    use State::*;

    match state {
        Alpha(a, b, c, Subscript::Iota) => Alpha(a, b, c, Subscript::None),
        Alpha(a, b, c, Subscript::None) => Alpha(a, b, c, Subscript::Iota),
        Eta(a, b, c, Subscript::Iota) => Eta(a, b, c, Subscript::None),
        Eta(a, b, c, Subscript::None) => Eta(a, b, c, Subscript::Iota),
        Omega(a, b, c, Subscript::Iota) => Omega(a, b, c, Subscript::None),
        Omega(a, b, c, Subscript::None) => Omega(a, b, c, Subscript::Iota),
        _ => state,
    }
}
fn toggle_diaeresis(state: State) -> State {
    use State::*;

    match state {
        Iota(a, b, c, Diaeresis::Some) => Iota(a, b, c, Diaeresis::None),
        Iota(a, b, c, Diaeresis::None) => Iota(a, b, c, Diaeresis::Some),
        Ypsilon(a, b, c, Diaeresis::Some) => Ypsilon(a, b, c, Diaeresis::None),
        Ypsilon(a, b, c, Diaeresis::None) => Ypsilon(a, b, c, Diaeresis::Some),
        _ => state,
    }
}

fn replace(buffer: &mut String, old: &str, new: &str) {
    let range = (buffer.len() - old.len())..(buffer.len());
    buffer.replace_range(range, new);
}

impl State {
    fn to_str(self) -> &'static str {
        use State::*;

        match self {
            Initial => "",
            Alpha(a, b, c, d) => uc::Alpha[(a, b, c, d)],
            Beta(a) => uc::Beta[a],
            Gamma(a) => uc::Gamma[a],
            Delta(a) => uc::Delta[a],
            Epsilon(a, b, c) => uc::Epsilon[(a, b, c)],
            Zeta(a) => uc::Zeta[a],
            Eta(a, b, c, d) => uc::Eta[(a, b, c, d)],
            Theta(a) => uc::Theta[a],
            Iota(a, b, c, d) => uc::Iota[(a, b, c, d)],
            Kappa(a) => uc::Kappa[a],
            Lambda(a) => uc::Lambda[a],
            Mu(a) => uc::Mu[a],
            Nu(a) => uc::Nu[a],
            Xi(a) => uc::Xi[a],
            Omicron(a, b, c) => uc::Omicron[(a, b, c)],
            Pi(a) => uc::Pi[a],
            Rho(a, b) => uc::Rho[(a, b)],
            Sigma(a) => uc::Sigma[a],
            Tau(a) => uc::Tau[a],
            Ypsilon(a, b, c, d) => uc::Ypsilon[(a, b, c, d)],
            Phi(a) => uc::Phi[a],
            Chi(a) => uc::Chi[a],
            Psi(a) => uc::Psi[a],
            Omega(a, b, c, d) => uc::Omega[(a, b, c, d)],
        }
    }

    #[inline]
    pub fn initial() -> Self {
        Self::Initial
    }

    #[inline]
    pub fn is_initial(self) -> bool {
        matches!(self, Self::Initial)
    }

    pub fn rewrite_buffer(self, c: u8, buffer: &mut String) -> Self {
        use State::*;

        let old = self;
        let new = transit(old, c);

        match old {
            Initial => buffer.push_str(new.to_str()),
            Sigma(a) => {
                if matches!(new, Initial) {
                    let sigma = old.to_str();
                    let final_sigma = uc::FinalSigma[a];
                    replace(buffer, sigma, final_sigma);
                } else if !is_diacritics(c) {
                    buffer.push_str(new.to_str());
                }
            }
            Alpha(_, _, _, _)
            | Epsilon(_, _, _)
            | Eta(_, _, _, _)
            | Iota(_, _, _, _)
            | Omicron(_, _, _)
            | Ypsilon(_, _, _, _)
            | Omega(_, _, _, _) => {
                if is_diacritics(c) {
                    let old = old.to_str();
                    let new = new.to_str();
                    replace(buffer, old, new);
                } else {
                    buffer.push_str(new.to_str());
                }
            }
            _ => {
                if !is_diacritics(c) {
                    buffer.push_str(new.to_str());
                }
            }
        }

        new
    }
}
