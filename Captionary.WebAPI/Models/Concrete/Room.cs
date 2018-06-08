using System.Collections.Generic;
using Captionary.Models.Abstract;

namespace Captionary.Models.Concrete
{
    public class Room : IRoom
    {
        string id;
        HashSet<Player> players;
        LinkedList<Round> rounds;

        public Room()
        {
            this.players = new HashSet<Player>();
            this.rounds = new LinkedList<Round>();
        }

        public string ID
        {
            get { return id; }
            set { this.id = value; }
        }

        public HashSet<Player> Players
        {
            get { return players; }
            set { this.players = value; }
        }

        public LinkedList<Round> Rounds
        {
            get { return rounds; }
            set { this.rounds = value; }            
        }

        public bool AddPlayer(Player player)
        {
            return this.players.Add(player);
        }

        public bool RemovePlayer(Player player)
        {
            return players.Remove(player);
        }

        public bool AddRound(Round round)
        {
            rounds.AddLast(round);

            return true;
        }
    }
}