export interface Car {
  brand: string;
  model: string;
  year: number;
}

console.log(
  "I wonder if wheelchair can be considered a type of car by definition"
);

class Wheelchair implements Car {
  brand: string;
  model: string;
  year: number;
}

// Why not?
