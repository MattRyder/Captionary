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

        public Room()
        {
            this.players = new HashSet<IPlayer>();
        }

        [ProtoMember(1)]
        public string ID
        {
            get { return id; }
            set { this.id = value; }
        }

        public bool AddPlayer(IPlayer player)
        {
            return this.players.Add(player);
        }

        public IEnumerable<IPlayer> GetPlayers()
        {
            return players;
        }

        public bool RemovePlayer(IPlayer player)
        {
            return players.Remove(player);
        }
    }
}