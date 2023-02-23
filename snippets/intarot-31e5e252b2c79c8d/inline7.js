export function angry_animate_top_card_left_facedown() {

    document.querySelector(".top_card").animate(
    [
      { transform: 'translateX(0) rotate(0deg) rotateY(180deg)' },
      { transform: 'translateX(-100px) rotate(10deg) rotateY(180deg)' },
      { transform: 'translateX(-50px) rotate(-10deg) rotateY(180deg)' },
      { transform: 'translateX(-70px) rotate(15deg) rotateY(180deg)' },
      { transform: 'translateX(-50px) rotate(-5deg) rotateY(180deg)' },
      { transform: 'translateX(0) rotate(0deg) rotateY(180deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }