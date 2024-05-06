// Ariful Haque example javascript file 2

export function weird_function(input) {

  let a = input;
  let b;
  if (a === 8) {
    b = 7
  }
  else {
    b = 9
  }

  let c = a * b;

  for (let i = 0; i < c; i++) {
    console.log(`Print ${i}`);
  }

  if (c > 30) {
    console.log("Greater than 30");
  }

  let chosen = a * 2;

  return chosen * 2 * 2;

}

console.log(weird_function(8));