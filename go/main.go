// Implementing the classic snake game in Go using SDL2
package main

import (
	"math/rand"

	"github.com/veandco/go-sdl2/sdl"
)

const (
	fps             = 20
	targetFrameTime = (1000 / fps)
	gridXSize       = 40
	gridYSize       = 30
	dotSize         = 25
)

type Direction int

const (
	Up Direction = iota
	Down
	Left
	Right
)

type State int

const (
	Playing State = iota
	Paused
	Dead
)

type Point struct {
	x int32
	y int32
}

type GameContext struct {
	snakeDirection Direction
	food           Point
	state          State
	run            bool
	score          int
	grow           bool
	lastFrameTime  uint64
	allowMove      bool
	snakeBody      []Point
}

func main() {
	if err := sdl.Init(sdl.INIT_VIDEO | sdl.INIT_EVENTS); err != nil {
		panic(err)
	}
	defer sdl.Quit()

	window, err := sdl.CreateWindow("Snake", sdl.WINDOWPOS_CENTERED, sdl.WINDOWPOS_CENTERED, gridXSize*dotSize, gridYSize*dotSize, sdl.WINDOW_SHOWN)
	if err != nil {
		panic(err)
	}
	defer window.Destroy()

	renderer, err := sdl.CreateRenderer(window, -1, sdl.RENDERER_ACCELERATED)
	if err != nil {
		panic(err)
	}
	defer renderer.Destroy()

	context := GameContext{Right, Point{5, 5}, Playing, true, 0, false, 0, true, []Point{{3, 3}}}

	for context.run {
		context.processInput()
		context.update()
		context.render(renderer)
	}

}

func (context *GameContext) processInput() {
	for event := sdl.PollEvent(); event != nil; event = sdl.PollEvent() {
		switch t := event.(type) {
		case *sdl.QuitEvent:
			context.run = false
		case *sdl.KeyboardEvent:
			if t.Type == sdl.KEYDOWN {
				switch t.Keysym.Sym {
				case sdl.K_ESCAPE:
					context.run = false
				case sdl.K_UP, sdl.K_w:
					if context.snakeDirection != Down && context.allowMove {
						context.snakeDirection = Up
						context.allowMove = false
					}
				case sdl.K_DOWN, sdl.K_s:
					if context.snakeDirection != Up && context.allowMove {
						context.snakeDirection = Down
						context.allowMove = false
					}
				case sdl.K_LEFT, sdl.K_a:
					if context.snakeDirection != Right && context.allowMove {
						context.snakeDirection = Left
						context.allowMove = false
					}
				case sdl.K_RIGHT, sdl.K_d:
					if context.snakeDirection != Left && context.allowMove {
						context.snakeDirection = Right
						context.allowMove = false
					}
				case sdl.K_SPACE:
					if context.state == Playing {
						context.state = Paused
					} else if context.state == Paused {
						context.state = Playing
					} else if context.state == Dead {
						context.state = Playing
						context.snakeDirection = Right
						context.food = Point{5, 5}
						context.score = 0
						context.grow = false
						context.allowMove = true
						context.snakeBody = []Point{{rand.Int31(), rand.Int31()}}
					}
				}
			}
		}
	}
}

func (context *GameContext) update() {
	timeToWait := targetFrameTime - (sdl.GetTicks64() - context.lastFrameTime)
	if timeToWait > 0 && timeToWait <= targetFrameTime {
		sdl.Delay(uint32(timeToWait))
	}
	if context.state == Paused || context.state == Dead {
		return
	}

	var nextHead Point

	switch context.snakeDirection {
	case Up:
		nextHead = Point{context.snakeBody[0].x, context.snakeBody[0].y - 1}
	case Down:
		nextHead = Point{context.snakeBody[0].x, context.snakeBody[0].y + 1}
	case Left:
		nextHead = Point{context.snakeBody[0].x - 1, context.snakeBody[0].y}
	case Right:
		nextHead = Point{context.snakeBody[0].x + 1, context.snakeBody[0].y}
	}

	if nextHead.x < 0 || nextHead.x >= gridXSize || nextHead.y < 0 || nextHead.y >= gridYSize {
		context.state = Dead
		return
	}

	for _, bodyPart := range context.snakeBody {
		if bodyPart == nextHead {
			context.state = Dead
			return
		}
	}

	context.grow = false

	if nextHead == context.food {
		context.score++
		context.grow = true
		context.food = Point{rand.Int31() % gridXSize, rand.Int31() % gridYSize}
	}

	context.snakeBody = append([]Point{nextHead}, context.snakeBody...)

	if !context.grow {
		context.snakeBody = context.snakeBody[:len(context.snakeBody)-1]
	}

	context.allowMove = true
	context.lastFrameTime = sdl.GetTicks64()
}

func (context *GameContext) render(renderer *sdl.Renderer) {
	switch context.state {
	case Playing:
		renderer.SetDrawColor(0, 0, 0, 255)
		renderer.Clear()
	default:
		renderer.SetDrawColor(30, 30, 30, 255)
		renderer.Clear()
	}

	for _, bodyPart := range context.snakeBody {
		renderer.SetDrawColor(0, 255, 0, 255)
		renderer.FillRect(&sdl.Rect{X: bodyPart.x * dotSize, Y: bodyPart.y * dotSize, W: dotSize, H: dotSize})
	}

	renderer.SetDrawColor(255, 0, 0, 255)

	renderer.FillRect(&sdl.Rect{X: context.food.x * dotSize, Y: context.food.y * dotSize, W: dotSize, H: dotSize})

	renderer.Present()
}
