using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
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

        public GameHub(IRepo<Room> roomRepo)
        {
            this._roomRepo = roomRepo;
        }

        public override async Task OnConnectedAsync()
        {
            Console.WriteLine("User connected to Captionary SignalR. CID: " + Context.ConnectionId);
            await Task.CompletedTask;
        }

        public override async Task OnDisconnectedAsync(Exception exception)
        {
            Player player = null; //await _playerRepo.FindAsync(Context.ConnectionId);
            if (player != null)
            {
                // _playerRepo.Delete(player);
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

            Room room = null;
            if (String.IsNullOrEmpty(roomId))
            {
                room = new Room();
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

            if (room.AddPlayer(player))
            {
                await _roomRepo.SaveAsync(room);

                await Groups.AddToGroupAsync(Context.ConnectionId, room.ID);

                await Clients.Caller.SendAsync("JoinGame", player.Name, room.ID);
                await Clients.Others.SendAsync("PlayerConnected", player.Name);
            }

            await _roomRepo.SaveAsync(room);

            if (room.Players.Count == 1)
            {
                await StartRound(room.ID);
            }
        }

        public async Task SendMessage(string roomId, JObject message)
        {
            var room = await _roomRepo.FindAsync(roomId);
            if (room == null)
                return;

            var player = room.Players.First(plyr => plyr.ID == Context.ConnectionId);
            if (player == null)
                return;

            Console.WriteLine(player.Name + "(" + player.ID + ") says: " + message["message"]);

            message["senderId"] = 0;
            await Clients.Caller.SendAsync("ReceiveMessage", message);

            message["senderId"] = player.ID;
            await Clients.Others.SendAsync("ReceiveMessage", message);
        }

        public async Task StartRound(string roomId)
        {
            var room = await _roomRepo.FindAsync(roomId);

            if (room == null)
            {
                return;
            }

            var round = new Round()
            {
                ImageUrl = "https://lorempixel.com/400/400/?_=" + Guid.NewGuid().ToString()
            };

            room.AddRound(round);
            await _roomRepo.SaveAsync(room);

            await Clients.Group(room.ID).SendAsync("RoundStarted", round.ID, round.ImageUrl);

        }

        public async Task SubmitCaption(string roomId, string roundId, string captionText)
        {
            var room = await _roomRepo.FindAsync(roomId);
            if (room == null)
                return;

            var player = room.Players.First(p => p.ID == Context.ConnectionId);
            if (player == null)
                return;

            var caption = new Caption()
            {
                PlayerID = player.ID,
                Text = captionText
            };

            var round = room.Rounds.Last.Value;
            round.SubmitCaption(caption);

            await _roomRepo.SaveAsync(room);

        }
    }
}