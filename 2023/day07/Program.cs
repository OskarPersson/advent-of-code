using System.Collections;
using System.Text.RegularExpressions;




// hand => [strength, bid]
var hands = new Dictionary<string, List<int>>();


var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

int Calc(List<char> strengths, Dictionary<string, List<int>> hands)
{
    var comparer = new MyComparer { Strengths = strengths.AsEnumerable().Reverse().ToList() };
    return hands.OrderBy(h => h, comparer).Select(hand => hand.Value[1]).Select((bid, i) => bid * (i + 1)).Sum();
}

int part1(IEnumerable<string> lines)
{
    var strengths = new List<char>()
    {
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
    };

    foreach (var l in lines)
    {
        var s = l.Split(" ");
        var hand = s[0];
        var bid = int.Parse(s[1]);

        hands[hand] = new List<int> { GetRank(hand), bid };
    }

    return Calc(strengths, hands);
}

int part2(IEnumerable<string> lines)
{
    var strengths = new List<char>()
    {
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
    };

    foreach (var l in lines)
    {
        var s = l.Split(" ");
        var hand = s[0];
        var bid = int.Parse(s[1]);

        var bestChar = strengths.MaxBy(c => GetRank(hand.Replace('J', c)));
        hands[hand] = new List<int> { GetRank(hand.Replace('J', bestChar)), bid };
    }

    return Calc(strengths, hands);
}

Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");

int GetRank(string hand)
{
    var handchars = hand.ToCharArray().ToList();
    handchars.Sort();

    var grp = handchars.GroupBy(x => x).ToList();
    if (grp.Count() == 1)
    {
        // five of a kind
        return 7;
    }
    else if (grp.Count() == 2)
    {
        var first = grp.First();
        var second = grp.Last();

        if (first.Count() == 1 || second.Count() == 1)
        {
            // four of a kind
            return 6;
        } else
        {
            // full house
            return 5;
        }
    } else if (grp.Count() == 3)
    {
        var ordered = grp.OrderBy(g => g.Count()).ToList();
        var first = ordered.First();
        var second = ordered[1];
        var third = ordered[2];

        if (third.Count() == 3)
        {
            // three of a kind
            return 4;
        } else if (first.Count() == 1 && second.Count() == 2 && third.Count() == 2)
        {
            // two pair
            return 3;
        }

        // Should not happen...
        throw new Exception("Unexpected hand");
    } else if (grp.Count() == 4)
    {
        // one pair
        return 2;
    } else
    {
        // high card
        return 1;
    }
}

public class MyComparer : IComparer<Object>
{
    public List<char> Strengths { get; set; }

    public int Compare(Object stringA, Object stringB)
    {
        var da = (KeyValuePair<string, List<int>>) stringA;
        var db = (KeyValuePair<string, List<int>>) stringB;
        
        if (da.Value[0] != db.Value[0]) {
            return da.Value[0].CompareTo(db.Value[0]);
        }

        var valueA = da.Key;
        var valueB = db.Key;

        var achars = valueA.ToCharArray().ToList();
        var bchars = valueB.ToCharArray().ToList();

        for (int i = 0; i < achars.Count; i++)
        {
            var a = achars[i];
            var b = bchars[i];
            if (a != b)
            {
                return Strengths.IndexOf(a).CompareTo(Strengths.IndexOf(b));
            }
        }

        return 0;
    }
}
