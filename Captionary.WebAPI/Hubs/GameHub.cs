using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Threading.Tasks;
using Captionary.Models.Abstract;
using Captionary.Models.Concrete;
using Captionary.WebAPI.Repos;
using Microsoft.AspNetCore.SignalR;
using Newtonsoft.Json.Linq;
using StackExchange.Redis.Extensions.Core;

namespace Captionary.WebAPI.Hubs
{
    public class GameHub : Hub
    {
        private readonly IRepo<Room> _roomRepo;
        private readonly IRepo<Player> _playerRepo;
        private readonly IRepo<Round> _roundRepo;

        public GameHub(IRepo<Room> roomRepo, IRepo<Player> playerRepo, IRepo<Round> roundRepo)
        {
            this._roomRepo = roomRepo;
            this._playerRepo = playerRepo;
            this._roundRepo = roundRepo;
        }

        public override async Task OnConnectedAsync()
        {
            Console.WriteLine("User connected to Captionary SignalR. CID: " + Context.ConnectionId);
            await Task.CompletedTask;
        }

        public override async Task OnDisconnectedAsync(Exception exception)
        {
            var player = await _playerRepo.FindAsync(Context.ConnectionId);
            if (player != null)
            {
                _playerRepo.Delete(player);
            }
            Console.WriteLine(player?.Name + "(" + Context.ConnectionId +
                ") disconnected from Captionary SignalR.");
        }

        public async Task PlayerLogin(string playerName, string roomId)
        {
            Console.WriteLine("Connection " + Context.ConnectionId + " logging in as: " + playerName);

            var player = new Player()
            {
                ID = Context.ConnectionId,
                Name = playerName
            };

            await _playerRepo.SaveAsync(player);

            Room room = null;
            if (String.IsNullOrEmpty(roomId))
            {
                room = new Room();

                await _roomRepo.SaveAsync(room);
            }
            else
            {
                room = await _roomRepo.FindAsync(roomId);
            }

            if (room == null)
            {
                return;
            }

            Console.WriteLine(player.Name + " is requesting access to Room " + room.ID);
            await Groups.AddToGroupAsync(Context.ConnectionId, room.ID);

            // await _cache.SetStringAsync(Context.ConnectionId, playerName);

            await Clients.Caller.SendAsync("JoinGame", player.Name, room.ID);
            await Clients.Others.SendAsync("PlayerConnected", player.Name);

            await StartRound(room.ID);
        }

        public async Task SendMessage(JObject message)
        {
            var player = await _playerRepo.FindAsync(Context.ConnectionId);
            if (player == null)
            {
                return;
            }

            Console.WriteLine(player.Name + "(" + player.ID + ") says: " + message["message"]);

            message["senderId"] = 0;
            await Clients.Caller.SendAsync("ReceiveMessage", message);

            message["senderId"] = player.ID;
            await Clients.Others.SendAsync("ReceiveMessage", message);
        }

        public async Task StartRound(string roomId)
        {
            var room = await _roomRepo.FindAsync(roomId);

            if(room == null)
            {
                return;
            }

            var round = new Round()
            {
                ImageUrl = "https://lorempixel.com/400/400/"
            };

            var roundPersisted = await _roundRepo.SaveAsync(round);

            if (roundPersisted)
            {
                room.AddRound(round);

                await Clients.Group(room.ID).SendAsync("RoundStarted", round.ID, round.ImageUrl);
            }
        }
    }
}