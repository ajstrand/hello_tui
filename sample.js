function greet(name) {
    console.log(`Hello, ${name}!`);
}

const numbers = [1, 2, 3, 4, 5];
const doubled = numbers.map(x => x * 2);

class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
    
    introduce() {
        return `Hi, I'm ${this.name} and I'm ${this.age} years old.`;
    }
}

// Arrow function
const add = (a, b) => a + b;

// Async/await example
async function fetchData() {
    try {
        const response = await fetch('/api/data');
        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching data:', error);
    }
}

greet('World');
