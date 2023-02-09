export function angry_animate_top_card_right() {

    document.querySelector(".top_card").animate(
    [
        { transform: 'translateX(0) rotate(0deg)' },
        { transform: 'translateX(100px) rotate(-10deg)' },
        { transform: 'translateX(50px) rotate(10deg)' },
        { transform: 'translateX(70px) rotate(-15deg)' },
        { transform: 'translateX(50px) rotate(5deg)' },
        { transform: 'translateX(0) rotate(0deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }