#include <stdio.h>
#include <stdbool.h>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

/*
global constant declarations
*/

const int FPS = 15;
const int TARGET_FRAME_TIME = (1000/FPS); 
const int GRID_X_SIZE = 40;
const int GRID_Y_SIZE = 30;
const int DOT_SIZE = 25;
const int FONT_SIZE = 25;
const char* FONT_PATH = "font.ttf";

/*
typedefs
*/

typedef struct {
  int x;
  int y;
} Point;

typedef struct {
  enum {Up, Right, Down, Left} snake_direction;
  Point food;
  enum {Playing, Paused, Dead} state;
  bool run;
  int score;
  bool grow;
  int last_frame_time;
  bool allow_move;
  int snake_body_size;
  Point snake_body[100];
} GameContext;

/*
function declarations
*/

void process_input(GameContext*);
void update(GameContext*);
void render(SDL_Renderer*, GameContext*, TTF_Font*);

int main(int argc, char* argv[]) {
  SDL_Init(SDL_INIT_VIDEO | SDL_INIT_EVENTS);
  TTF_Init();
  TTF_Font* font = TTF_OpenFont(FONT_PATH, FONT_SIZE);

  SDL_Window* win = SDL_CreateWindow(
    "Snake",
    SDL_WINDOWPOS_CENTERED,
    SDL_WINDOWPOS_CENTERED,
    GRID_X_SIZE * DOT_SIZE,
    GRID_Y_SIZE * DOT_SIZE,
    SDL_WINDOW_OPENGL
  );

  SDL_Renderer* renderer = SDL_CreateRenderer(win, 0, SDL_RENDERER_ACCELERATED); // 0 = OpenGL driver per below

  /*
  for (int i = 0; i < 3; i++) {
  SDL_RendererInfo info;
  SDL_GetRenderDriverInfo(i, &info);
  printf("%s, %d\n",info.name,i);
  }
  */

  GameContext context = {
    Right,                      // snake_direction
    {5,5},                      // food
    Playing,                    // state
    true,                       // run
    0,                          // score
    false,                      // grow
    0,                          // last_frame_time
    true,                       // allow_move
    1,                          // snake_body_size
    malloc(sizeof(Point) * 100) // snake_body
  };

  context.snake_body[0].x = 3;
  context.snake_body[0].y = 3;

  while (context.run) {
    process_input(&context);

    update(&context);

    render(renderer, &context, font);
  }

  SDL_DestroyRenderer(renderer);
  TTF_CloseFont(font);
  SDL_DestroyWindow(win);
  TTF_Quit();
  SDL_Quit();

  return 0;
}

void process_input(GameContext* context) {
  SDL_Event event;

  if (context->state == Paused) {
    context->allow_move = false;
  }
  while (SDL_PollEvent(&event)) {
    switch (event.type) {
      case SDL_QUIT:
        context->run = false;
        break;
      case SDL_KEYDOWN:
        if (event.key.keysym.sym == SDLK_UP || event.key.keysym.sym == SDLK_w) {
          if (context->allow_move && context->snake_direction != Down) {
            context->snake_direction = Up;
            context->allow_move = false;
          }
        } else if (event.key.keysym.sym == SDLK_DOWN || event.key.keysym.sym == SDLK_s) {
          if (context->allow_move && context->snake_direction != Up) {
            context->snake_direction = Down;
            context->allow_move = false;
          }
        } else if (event.key.keysym.sym == SDLK_LEFT || event.key.keysym.sym == SDLK_a) {
          if (context->allow_move && context->snake_direction != Right) {
            context->snake_direction = Left;
            context->allow_move = false;
          }
        } else if (event.key.keysym.sym == SDLK_RIGHT || event.key.keysym.sym == SDLK_d) {
          if (context->allow_move && context->snake_direction != Left) {
            context->snake_direction = Right;
            context->allow_move = false;
          }
        } else if (event.key.keysym.sym == SDLK_ESCAPE) {
          (context->state == Playing) ? (context->state = Paused) : (context->run = false);
        } else if (event.key.keysym.sym == SDLK_SPACE) {
          switch (context->state) {
            case Paused:
              context->state = Playing;
              break;
            case Dead:
              context->snake_body_size = 1;
              context->snake_direction = Right;
              context->snake_body[0].x = 3;
              context->snake_body[0].y = 3;
              context->score = 0;
              context->state = Playing;
              context->food.x = rand() % GRID_X_SIZE;
              context->food.y = rand() % GRID_Y_SIZE;
              break;
            default: 
              break;
          }
        }
      break;
    }
  }
}

void update(GameContext* context) {
  int time_to_wait = TARGET_FRAME_TIME - (SDL_GetTicks() - context->last_frame_time);

  if (time_to_wait > 0 && time_to_wait <= TARGET_FRAME_TIME) {
    SDL_Delay(time_to_wait);
  }
  if (context->state == Paused || context->state == Dead) {
    return;
  }
  Point next_head_position;
  switch (context->snake_direction) {
    case Up:
      next_head_position.x = context->snake_body->x;
      next_head_position.y = context->snake_body->y - 1;
      break;
    case Down:
      next_head_position.x = context->snake_body->x;
      next_head_position.y = context->snake_body->y + 1;
      break;
    case Left:
      next_head_position.x = context->snake_body->x - 1;
      next_head_position.y = context->snake_body->y;
      break;
    case Right:
      next_head_position.x = context->snake_body->x + 1;
      next_head_position.y = context->snake_body->y;
      break;
  }
  if (next_head_position.x > (GRID_X_SIZE - 1)
      || next_head_position.x < 0
      || next_head_position.y > (GRID_Y_SIZE - 1)
      || next_head_position.y < 0) {
    context->state = Dead;
    return;
  }
  for (int i = 0; i < context->snake_body_size; i++) {
    if (next_head_position.x == context->snake_body[i].x && next_head_position.y == context->snake_body[i].y) {
      context->state = Dead;
      return;
    }
  }

  context->grow = false;

  if (next_head_position.x == context->food.x && next_head_position.y == context->food.y) {
    context->grow = true;
    context->food.x = rand() % GRID_X_SIZE;
    context->food.y = rand() % GRID_Y_SIZE;
    context->score++;
  }
  for (int i = context->snake_body_size-1; i >= 0; i--) {
    context->snake_body[i+1] = context->snake_body[i];
  }

  context->snake_body[0] = next_head_position;

  if (context->grow) {
    context->snake_body_size++;
  }

  context->allow_move = true;
  context->last_frame_time = SDL_GetTicks();
  }

void render(SDL_Renderer* renderer, GameContext* context, TTF_Font* font) {
  switch (context->state) {
    case Playing:
      SDL_SetRenderDrawColor(renderer, 0, 0, 0, SDL_ALPHA_OPAQUE);
      SDL_RenderClear(renderer);
      break;
    default:
      SDL_SetRenderDrawColor(renderer, 30, 30, 30, SDL_ALPHA_OPAQUE);
      SDL_RenderClear(renderer);
      break;
  }

  for (int i = 0; i < context->snake_body_size; i++) {
    (context->snake_body_size > 3 && i % 2 == 1) ? (SDL_SetRenderDrawColor(renderer, 0, 150, 0, SDL_ALPHA_OPAQUE)) : (SDL_SetRenderDrawColor(renderer, 0, 255, 0, SDL_ALPHA_OPAQUE));
    SDL_Rect snake_rect = {
      context->snake_body[i].x * DOT_SIZE,
      context->snake_body[i].y * DOT_SIZE,
      DOT_SIZE,
      DOT_SIZE
    };
    SDL_RenderFillRect(renderer, &snake_rect);
  }

  SDL_SetRenderDrawColor(renderer, 255, 0, 0, SDL_ALPHA_OPAQUE);

  SDL_Rect food_rect = {
    context->food.x * DOT_SIZE,
    context->food.y * DOT_SIZE,
    DOT_SIZE,
    DOT_SIZE
  };

  SDL_RenderFillRect(renderer, &food_rect);

  SDL_Color white = {255, 255, 255, 255};

  char score_str[12];
  snprintf(score_str, 12, "Score: %d", context->score);

  SDL_Surface* font_surface = TTF_RenderText_Solid(font, score_str, white);
  SDL_Texture* font_texture = SDL_CreateTextureFromSurface(renderer, font_surface);
  int font_width, font_height;
  SDL_QueryTexture(font_texture, NULL, NULL, &font_width, &font_height);

  SDL_Rect font_rect = {0, 0, font_width, font_height};
  SDL_RenderCopy(renderer, font_texture, NULL, &font_rect);
  SDL_RenderPresent(renderer);
  SDL_DestroyTexture(font_texture);
}
