fn main() {
    {
        let <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="2">String::from</tspan>("hello"); // s is valid from this point forward
    
        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}