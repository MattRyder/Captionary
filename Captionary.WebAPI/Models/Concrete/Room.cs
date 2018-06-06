using System;
using System.Collections.Generic;
using Captionary.Models.Abstract;
using ProtoBuf;

namespace Captionary.Models.Concrete
{
    [ProtoContract]
    public class Room : IRoom
    {
        string id;
        HashSet<IPlayer> players;
        LinkedList<IRound> rounds;

        public Room()
        {
            this.players = new HashSet<IPlayer>();
            this.rounds = new LinkedList<IRound>();
        }

        [ProtoMember(1)]
        public string ID
        {
            get { return id; }
            set { this.id = value; }
        }

        public HashSet<IPlayer> Players
        {
            get { return players; }
            set { this.players = value; }
        }

        public LinkedList<IRound> Rounds
        {
            get { return rounds; }
            set { this.rounds = value; }            
        }

        public bool AddPlayer(IPlayer player)
        {
            return this.players.Add(player);
        }

        public bool RemovePlayer(IPlayer player)
        {
            return players.Remove(player);
        }

        public bool AddRound(IRound round)
        {
            rounds.AddLast(round);

            return true;
        }
    }
}