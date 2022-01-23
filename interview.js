var a = 100;
function fn() {
  var a;
  alert(a); // undefined
  a = 200;
  alert(a); // 200
}
fn();
alert(a); //100
var a;
alert(a); // 100
var a = 300;
alert(a); // 300

var obj1 = {
  name: "obj1",
  fn: function () {
    console.log(this.name);
  },
};
var obj2 = { name: "obj2" };
var obj3 = { name: "obj3" };
obj1.fn(); // obj1
var newFn = obj1.fn;
newFn(); // undefined
newFn.call(obj2); // obj2
obj3.fn = newFn;
obj3.fn(); // obj3

function myPromiseAll(promiseArr) {
  return new Promise((resolve, reject) => {
    let count = 0;
    const resArr = [];
    promiseArr.forEach((item) => {
      item
        .then((res) => {
          count++;
          resArr.push(res);
          if (count === promiseArr.length) {
            resolve(resArr);
          }
        })
        .catch((err) => {
          reject(err);
        });
    });
  });
}
//    i
// 9999 9999
//      3333
// 9999

// 15432
// 1
function addBigNumber(number1Str, number2Str) {
  let res = "";
  let jinwei = 0;
  let i = Math.min(number1Str.length > number2Str.length);
  for (; i >= 0; i--) {
    let add = Number(number1Str[i]) + Number(number2Str[i]) + jinwei;
    jinwei = add >= 10 ? (add / 10).toFixed(0) : 0;
    add = add % 10;
    res = add + res;
  }
  let k = Math.abs(number1Str.length - number2Str.length);
  let shengyuStr =
    number1Str.length > number2Str.length
      ? number1Str.slice(0, k + 1)
      : number2Str.slice(0, k);
  // jinwei 0 为 退出条件

  while (jinwei) {
    //
  }
  res += shengyuStr;
  return res;
}



// a c f d e b
// macro task 
// micro task 

console.log("a");
setTimeout(() => {
  console.log("b");
}, 0);
console.log("c");
Promise.resolve()
  .then(() => {
    console.log("d");
  })
  .then(() => {
    console.log("e");
  });

console.log("f");
