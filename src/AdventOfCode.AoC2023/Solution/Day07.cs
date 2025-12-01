using AdventOfCode.Core;

namespace AdventOfCode.AoC2023.Solution;

public class Day07 : ISolution
{
    public string Name => "Camel Cards";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var hands = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(ParseInputLine)
            .ToList();
        
        hands.Sort();

        return hands.Select((hand, i) => (i + 1) * hand.Bid)
            .Sum()
            .ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile).Replace('J', 'j');
        var hands = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(ParseInputLine)
            .ToList();
        
        hands.Sort();

        return hands.Select((hand, i) => (i + 1) * hand.Bid)
            .Sum()
            .ToString();
    }

    private static Hand ParseInputLine(string line)
    {
        var parts = line.Split(' ',  StringSplitOptions.TrimEntries);
        var bid = int.Parse(parts.Last());
        var cards = parts.First()
            .Select(Card.ParseCard)
            .ToList();
        
        return new Hand(bid, cards);
    }
}

public class Hand(int bid, List<CardType> cards) : IEquatable<Hand>, IComparable<Hand>
{
    public List<CardType> Cards => cards;
    
    public int Bid => bid;
    
    public HandType GetHandType()
    {
        var cardFrequency = Card.GetCardFrequency(cards);
        var maxFrequency = cardFrequency.Values.Max();

        return maxFrequency switch
        {
            5 => HandType.FiveOfAKind,
            4 =>  HandType.FourOfAKind,
            3 => cardFrequency.Values.Any(v => v == 2) 
                ? HandType.FullHouse 
                : HandType.ThreeOfAKind,
            2 => cardFrequency.Values.Count(v => v == 2) == 2
                ? HandType.TwoPair
                : HandType.OnePair,
            1 => HandType.HighCard,
            _ => throw new NotSupportedException("Should not happen")
        };
    }

    public bool Equals(Hand? other)
    {
        return Cards.SequenceEqual(other!.Cards);
    }

    public override bool Equals(object? obj)
    {
        if (obj is null) return false;
        if (ReferenceEquals(this, obj)) return true;
        
        return obj.GetType() == GetType() && Equals((Hand)obj);
    }

    public override int GetHashCode() =>
        Cards.GetHashCode() ^ bid.GetHashCode();

    public int CompareTo(Hand? other)
    {
        if (GetHandType() != other!.GetHandType())
            return GetHandType().CompareTo(other.GetHandType());
        
        return Cards
            .Zip(other!.Cards, (card, otherCard) => card.CompareTo(otherCard))
            .FirstOrDefault(x => x != 0);
    }
}

[Flags]
public enum HandType
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

public static class Card
{
    public static CardType ParseCard(char card)
    {
        return card switch
        {
            'j' => CardType.Joker,
            '2' => CardType.Two,
            '3' => CardType.Three,
            '4' => CardType.Four,
            '5' => CardType.Five,
            '6' => CardType.Six,
            '7' => CardType.Seven,
            '8' => CardType.Eight,
            '9' => CardType.Nine,
            'T' => CardType.Ten,
            'J' => CardType.Jack,
            'Q' => CardType.Queen,
            'K' => CardType.King,
            'A' => CardType.Ace,
            _ => throw new ArgumentException($"{card} is not a valid card")
        };
    }

    public static Dictionary<CardType, int> GetCardFrequency(List<CardType> cards)
    {
        var map = new Dictionary<CardType, int>();
        foreach (var card in cards.Where(card => !map.TryAdd(card, 1)))
            map[card]++;

        if (!map.TryGetValue(CardType.Joker, out var jokerCount))
            return map;
       
        var maxNoJoker = map.Where(pair => pair.Key != CardType.Joker)
            .OrderByDescending(pair => pair.Value)
            .ThenBy(x => x.Key)
            .FirstOrDefault();

        if (maxNoJoker.Key == CardType.Joker) 
            return map;
        
        map[maxNoJoker.Key] += jokerCount;
        map[CardType.Joker] = 0;

        return map;
    }
}

[Flags]
public enum CardType
{
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}