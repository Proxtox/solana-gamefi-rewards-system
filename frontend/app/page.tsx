"use client";

import { useState } from 'react';

export default function RewardsDashboard() {
  const [player, setPlayer] = useState('');

  const handleClaim = () => {
    alert(`Claiming rewards for ${player} (demo)`);
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-8 border rounded-3xl">
      <h1 className="text-3xl font-bold mb-8 text-center">GameFi Rewards</h1>

      <div className="space-y-4">
        <div>
          <label className="block text-sm mb-1">Player Address / ID</label>
          <input
            type="text"
            value={player}
            onChange={(e) => setPlayer(e.target.value)}
            className="w-full p-4 border rounded-2xl"
            placeholder="Player wallet or username"
          />
        </div>

        <button
          onClick={handleClaim}
          className="w-full bg-orange-600 hover:bg-orange-700 text-white py-4 rounded-2xl text-lg font-semibold"
        >
          Claim Rewards
        </button>
      </div>

      <p className="text-center text-sm text-gray-500 mt-6">Demo UI for Solana GameFi rewards system</p>
    </div>
  );
}