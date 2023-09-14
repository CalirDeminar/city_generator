console.log('Startup');
let menus = document.getElementsByClassName('collapsable_toggle');
let i = 0;
while (i < menus.length) {
  let menu = menus[i];
  menu.addEventListener('click', (event) => {
    let target = event.currentTarget.getElementsByClassName('collapse')[0];
    console.log(target.style.display);
    if (target.style.display === 'none') {
      console.log('setBlock');
      target.style.display = 'block';
    } else {
      console.log('setHidden');
      target.style.display = 'none';
    }
  });
  i += 1;
}
