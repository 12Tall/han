var addon = require('.');
addon.test_threadsafe_function((err, ...values) => {
  var i = 0;
  while (i++ < 10) {
    console.log(">> 子线程 <<")
  }
});
setInterval(() => {
  console.log(" > 主线程 <")
}, 100);