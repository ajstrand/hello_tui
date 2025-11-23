// Test JavaScript file for linting
function example() {
    var oldStyle = "avoid var";  // Should suggest let/const
    console.log("Debug message");  // Should suggest removal
    
    if (value == null) {  // Should suggest ===
        return false;
    }
    
    let longLine = "This is a very long line that exceeds 100 characters and should trigger the long line linting rule";
    
    function(){}  // Bad spacing
    
    let trailing = "space at end";     
}
