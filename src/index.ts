type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

function append(items: Item[]) {
  items.push("Hello Fem!");
}

const items: Item[] = [];

console.log(items);
append(items);
console.log(items);

const numbers: number[] = [];
console.log(numbers);
append(numbers);
console.log(numbers);
