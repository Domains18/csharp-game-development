using System;
using System.Diagnostics;
using System.Threading;

class Program
{
    static void Main()
    {
        Console.CursorVisible = false; //hide the blinking cursor
        Console.SetWindowSize(50, 22);

        Game game = new Game();
        while (game.IsRunning)
        {
            game.Run();
            Thread.Sleep(10);
        }
    }
}


class Player
{
    public float X { get; set; }
    public float Y { get; set; }
    public float XVelocity { get; set; }
    public float YVelocity { get; set; }
    public const float Speed = 15f;
    public const float size = 1f;
}


class Coin { public int X; public int Y; }

class Game
{
    private Player player;
    private Coin coin;
    private int score;
    private Random random = new Random();

    public bool IsRunning { get; private set; } = true;

    // arena
    private const int ArenaLeft = 2;
    private const int ArenaTop = 4;
    private const int ArenaWidth = 30;
    private const int ArenaHeight = 15;

    private const float FixedDeltaTime = 1f / 60f; //60 updates per secons
    private Stopwatch stopwatch = new Stopwatch();
    private float accumulator = 0f;


    public Game()
    {

        player = new Player { X = 10, Y = 5 };
        coin = new Coin { X = random.Next(ArenaLeft + ArenaWidth + ArenaLeft), Y = random.Next(ArenaTop, ArenaHeight + ArenaTop) };
        RespawnCoin();
        score = 0;
    }

    public void Run()
    {
        float frameTime = (float)stopwatch.Elapsed.TotalSeconds;
        stopwatch.Restart();

        if (frameTime > 0.25f) frameTime = 0.25f;

        accumulator += frameTime;
        //fixed timestep: run physics at consistent rate
        while (accumulator >= FixedDeltaTime)
        {
            HandleInput();
            Update(FixedDeltaTime);
            accumulator -= FixedDeltaTime;
        }

        Render();
    }

    public void Stop()
    {
        IsRunning = false;
    }


    private void HandleInput()
    {
        player.XVelocity = 0;
        player.YVelocity = 0;

        if (Console.KeyAvailable)
        {
            ConsoleKeyInfo keyInfo = Console.ReadKey(true);
            ConsoleKey key = keyInfo.Key;

            switch (key)
            {
                case ConsoleKey.UpArrow: player.YVelocity = -Player.Speed; break;
                case ConsoleKey.DownArrow: player.YVelocity = Player.Speed; break;
                case ConsoleKey.LeftArrow: player.XVelocity = -Player.Speed; break;
                case ConsoleKey.RightArrow: player.XVelocity = Player.Speed; break;
            }
        }

    }

    private void Update(float dt)
    {
        //apply velocity: position += velocity *time;
        player.X += player.XVelocity * dt;
        player.Y += player.YVelocity * dt;

        //arena bounds
        float minX = ArenaLeft;
        float maxX = ArenaLeft + ArenaWidth - 1;
        float minY = ArenaTop;
        float maxY = ArenaTop + ArenaHeight - 1;

        if (player.X < minX) player.X = minX;
        if (player.X > maxX) player.X = maxX;
        if (player.Y < minY) player.Y = minY;
        if (player.Y > maxY) player.Y = maxY;

        int playerIntX = (int)Math.Round(player.X);
        int playerIntY = (int)Math.Round(player.Y);


        if (playerIntX == coin.X && playerIntY == coin.Y)
        {
            score++;
            RespawnCoin();
        }
    }

    private void RespawnCoin()
    {
        do
        {
            coin = new Coin { X = random.Next(ArenaLeft, ArenaLeft + ArenaWidth), Y = random.Next(ArenaTop, ArenaTop + ArenaHeight) };
        } while ((int)Math.Round(player.X) == coin.X && (int)Math.Round(player.Y) == coin.Y);
    }


    private void Render()
    {
        Console.SetCursorPosition(0, 0);
        Console.WriteLine("╔══════════════════════════════════════════════╗");
        Console.WriteLine($"║  SCORE: {score,-4}  Arrows to move  Q to quit ║");
        Console.WriteLine("╚══════════════════════════════════════════════╝");

        for (int y = 0; y < ArenaTop + ArenaHeight; y++)
        {
            for (int x = 0; x < ArenaLeft + ArenaWidth + 2; x++)
            {
                bool isleftBorder = x == ArenaLeft - 1 && y >= ArenaTop && y < ArenaTop + ArenaHeight;
                bool isRightBorder = x == ArenaLeft + ArenaWidth && y >= ArenaTop && y < ArenaTop + ArenaHeight;
                bool isTopBorder = y == ArenaTop - 1 && x >= ArenaLeft && x < ArenaLeft + ArenaWidth;
                bool isBottomBorder = y == ArenaTop + ArenaHeight && x >= ArenaLeft && x < ArenaLeft + ArenaWidth;
                bool isCorner = (x == ArenaLeft - 1 || x == ArenaLeft + ArenaWidth) && (y == ArenaTop - 1 || y == ArenaTop + ArenaHeight);
                bool insideArena = x >= ArenaLeft && x < ArenaLeft + ArenaWidth && y >= ArenaTop && y < ArenaTop + ArenaHeight;

                int playerintX = (int)Math.Round(player.X);
                int playerintY = (int)Math.Round(player.Y);

                if (isCorner)
                    Console.Write("╬");
                else if (isleftBorder || isRightBorder)
                    Console.Write("║");
                else if (isTopBorder || isBottomBorder)
                    Console.Write("═");
                else if (insideArena)
                {
                    if (x == playerintX && y == playerintY)
                        Console.Write("☻");  // Player
                    else if (x == coin.X && y == coin.Y)
                        Console.Write("◆");  // Coin
                    else
                        Console.Write(" ");   // Empty space
                }
                else
                {
                    Console.Write(" ");
                }
            }
            Console.WriteLine();
        }
    }
}
