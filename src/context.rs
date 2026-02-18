pub enum Context {
    Hole,
    If1C, // (if C e e)
    If2C, // (if e C e)
    If3C, // (if e e C)
    AppC, // (e .... C e ....) 
}

// ex: (+ 1 (if (+ 2 []) 3 4)) -> AppC([+ 1] (If1C (AppC [+ 2] Hole []) 3 4) [])