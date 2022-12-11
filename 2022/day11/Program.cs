using System.Data.SqlTypes;
using System.Numerics;
using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();


List<Monkey> ParseMonkeys(List<string> lines) {
   var operationPattern = @"(\d+)";
   var divisiblePattern = @"(\d+)";
   var ifBoolPattern = @"(\d+)";
   var monkeys = lines.Chunk(7).Select(c =>
   {
      var mLines = c.Skip(1).Take(5).Select(ml => ml.Trim()).ToList();
      var startItems = mLines[0].Remove(0, 16).Split(", ").Select(s => BigInteger.Parse(s)).ToList();
      var operationSplit = mLines[1].Remove(0, 17).Split(" ").ToList();
      var operation1 = operationSplit[0];
      var operation2 = operationSplit[1];
      var operation3 = operationSplit[2];
      var operation = (operation1, operation2, operation3);
      
      var divisibleBy = int.Parse(Regex.Match(mLines[2], divisiblePattern).Groups[1].ToString());
      var ifTrue = int.Parse(Regex.Match(mLines[3], ifBoolPattern).Groups[1].ToString());
      var ifFalse = int.Parse(Regex.Match(mLines[4], ifBoolPattern).Groups[1].ToString());

      
      return new Monkey(
         startItems,
         operation,
         divisibleBy,
         ifTrue,
         ifFalse
      );
   }).ToList();
   return monkeys;
}

BigInteger Part1(List<string> lines)
{
   var monkeys = ParseMonkeys(lines);
   for (int i = 0; i < 20; i++)
   {
      foreach (var monkey in monkeys)
      {
         foreach (var item in monkey.StartingItems)
         {
            var worry = monkey.Operate(item);
            worry = Convert.ToInt32(Math.Floor((double)worry / 3));
            if (worry % monkey.DivisibleBy == 0)
            {
               monkeys[Convert.ToInt32(monkey.IfTrueMonkey)].StartingItems.Add(worry);
            }
            else
            {
               monkeys[Convert.ToInt32(monkey.IfFalseMonkey)].StartingItems.Add(worry);
            }
            monkey.Inspected++;
         }

         monkey.StartingItems = new List<BigInteger>();
      }
   }

      
   var best = monkeys.Select(m => m.Inspected).OrderDescending().Take(2).ToList();
   return best[0] * best[1];
}

BigInteger Part2(List<string> lines)
{
   var monkeys = ParseMonkeys(lines);
   var modulo = monkeys.Select(m => m.DivisibleBy).Aggregate((total, next) => total * next) ;

   for (int i = 0; i < 10000; i++)
   {
      foreach (var monkey in monkeys)
      {
         foreach (var item in monkey.StartingItems)
         {
            var worry = monkey.Operate(item);
            // Before you ask. No, I did not figure this out myself
            worry %= modulo;

            if (worry % monkey.DivisibleBy == 0)
            {
               monkeys[Convert.ToInt32(monkey.IfTrueMonkey)].StartingItems.Add(worry);
            }
            else
            {
               monkeys[Convert.ToInt32(monkey.IfFalseMonkey)].StartingItems.Add(worry);
            }
            monkey.Inspected++;
         }

         monkey.StartingItems = new List<BigInteger>();
      }
   }

      
   var best = monkeys.Select(m => m.Inspected).OrderDescending().Take(2).ToList();
   return best[0] * best[1];
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");

class Monkey
{
   public Monkey(List<BigInteger> startingItems, (string, string, string) operation, int divisibleBy, int ifTrueMonkey, int ifFalseMonkey)
   {
      Inspected = 0;
      StartingItems = startingItems;
      Operation = operation;
      DivisibleBy = divisibleBy;
      IfTrueMonkey = ifTrueMonkey;
      IfFalseMonkey = ifFalseMonkey;
   }

   public BigInteger Inspected { get; set; }
   public List<BigInteger> StartingItems { get; set; }
   public (string, string, string) Operation { get; set; }
   public int DivisibleBy { get; set; }
   public int IfTrueMonkey { get; set; }
   public int IfFalseMonkey { get; set; }

   public BigInteger Operate(BigInteger item)
   {
      var arg1 = Operation.Item1 == "old" ? item : int.Parse(Operation.Item1);
      var arg2 = Operation.Item3 == "old" ? item : int.Parse(Operation.Item3);

      if (Operation.Item2 == "+")
      {
         return arg1 + arg2;
      }

      return arg1 * arg2;
   }
}

