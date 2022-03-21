#include <SDL2/SDL.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

typedef struct Point {
  int x;
  int y;
} Point;

typedef struct Circ {
  Point origo;
  double r;
} Circle;

Circle circ_construct(int x, int y, double r) {
  Circle c = {
    .origo.x = x,
    .origo.y = y,
    .r       = r,
  };
  return c;
}

double distance(Point a, Point b) {
  return sqrt((a.x-b.x) * (a.x-b.x) + (a.y-b.y) * (a.y-b.y));
}

void set_circle(Circle *c, Point p, double r) {
  c->origo.x = p.x;
  c->origo.y = p.y;
  c->r	     = r;
}

void set_point(Point *p, int x, int y) {
  p->x = x;
  p->y = y;
}

double area(Circle c) {
  return c.r * c.r * M_PI;
}

bool in_circle(Point p, Circle c) {
  bool ret = false;
  if(distance(c.origo, p) < c.r) ret = true;
  return ret;
}

// heron formula
double area_trig(double a, double b) {
  double p = (a * 2 + b) / 2;
  return sqrt(p * (p-a) * (p-a) * (p-b));
}

double area_seq(double r, double alpha) {
  return (r * r) * alpha;
}

void get_angles(double *alpha, double *beta,
		Circle cA, Circle cB) {
  

  *beta = asin((cA.r / 2) / cB.r) * 2;
  *alpha = (M_PI - (*beta * 2));
}

double area_cB(Circle cA, Circle cB) {
  double alpha	= 0;
  double beta	= 0;
  double trig	= 0;
  double seqA	= 0;
  double seqB	= 0;
  
  get_angles(&alpha, &beta, cA, cB);
  trig = area_trig(cA.r, cB.r);
  seqA = area_seq(cA.r, alpha);
  seqB = area_seq(cB.r, beta);

  return (2 * seqB + 2 * seqA - 2 * trig);
}

#define WINDOW_WIDTH 800
#define CIRC_R (WINDOW_WIDTH / 2)

int main(void) {
  Circle A = circ_construct(300, 300, 100);
  Circle B = circ_construct(400, 300, 100);

  printf("-------------\n");
  printf("distance\n");
  A.origo.x = 450;
  A.origo.y = 330;
  B.origo.x = 880;
  B.origo.y = 300;
  printf("x1 450 y1 330    x2 880 y2 300\n");
  double d = distance(A.origo, B.origo);
  assert(d > 430 && d < 432);
  printf("d: %.2lf\n", d);

  printf("derekszogu: \n");
  A.origo.x = 300;
  A.origo.y = 300;
  A.r	    = 30;

  B.origo.x = 330;
  B.origo.y = 300;
  B.r       = sqrt(2) * A.r;

  double alpha = 0;
  double beta = 0;
  double trig = 0;
  double seqA = 0;
  double seqB = 0;
  get_angles(&alpha, &beta, A, B);
  trig = area_trig(A.r, B.r);
  seqA = area_seq(A.r, alpha);
  seqB = area_seq(B.r, alpha);

  printf("alpha: %.2lf r, %.2lf deg\n", alpha, alpha / M_PI * 180);
  printf("beta:  %.2lf r, %.2lf deg\n", beta, beta / M_PI * 180);
  printf("trig:  %.2f\n", trig);
  printf("seqA:  %.2f\n", seqA);
  printf("seqB:  %.2f\n", seqB);

  printf("B.r / 2             : %.2lf\n", B.r / 2);
  printf("(B.r / 2) / A.r     : %.2lf\n", (B.r/2) / A.r);
  printf("(B.r / 2) / A.r * 2 : %.2lf\n", ((B.r/2) / A.r) * 2);
  printf("alpha:              : %.2lf\n", asin(((B.r/2) / A.r)) * 2);
  return 0;
}

int main2(void){

  SDL_Event event;
  SDL_Renderer *renderer;
  SDL_Window *window;

  assert(!SDL_Init(SDL_INIT_VIDEO));
  SDL_CreateWindowAndRenderer(WINDOW_WIDTH, WINDOW_WIDTH, 0, &window, &renderer);
  SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
  SDL_RenderClear(renderer);

  Point cursor = { 0 };
  Circle cA = { 0 };
  Circle mA = { 0 };
  Circle cB = { 0 };
  Circle mB = { 0 };
  set_point(&cursor, WINDOW_WIDTH / 2, WINDOW_WIDTH / 2);
  set_circle(&cA, cursor, CIRC_R);
  set_point(&cursor, WINDOW_WIDTH / 2, WINDOW_WIDTH / 2);
  set_circle(&mA, cursor, 10);
  set_point(&cursor, WINDOW_WIDTH / 2 + cA.r, WINDOW_WIDTH / 2);
  set_circle(&cB, cursor, CIRC_R);
  set_point(&cursor, WINDOW_WIDTH / 2 + cA.r, WINDOW_WIDTH / 2);
  set_circle(&mB, cursor, 10);
  bool grow = true;
  while (1) {
    
    while(grow) {
      for(int y = 0; y < WINDOW_WIDTH; y++){
	for(int x = 0; x < WINDOW_WIDTH; x++){
	  cursor.x = x;
	  cursor.y = y;
	  Uint8 red = 255;
	  Uint8 green = 255;
	  Uint8 blue = 255;
	  if(in_circle(cursor, cA)) {
	    red = 0;
	  }
	  if(in_circle(cursor, cB)) {
	    green = 0;
	  }
	  if(in_circle(cursor, mA) || in_circle(cursor, mB)){
	    blue = 0;
	  }
	  SDL_SetRenderDrawColor(renderer, red, green, blue, 255);
	  SDL_RenderDrawPoint(renderer, x, y);
	}
      }
      SDL_RenderPresent(renderer);

      printf("areas: %.2lf %.2lf\n", area(cA), area_cB(cA, cB));
      if(area(cA) < area_cB(cA, cB)) grow = false;
      cB.r++;
    }
    /* END MAIN */
    if (SDL_PollEvent(&event) && event.type == SDL_QUIT)
      break;
  }

  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return EXIT_SUCCESS;
}

