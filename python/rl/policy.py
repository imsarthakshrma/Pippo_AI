# Custom neural network policy for Pippo agents.
import torch.nn as nn
from stable_baselines3.common.policies import ActorCriticPolicy

class PippoActorCriticPolicy(ActorCriticPolicy):
    """
    Custom policy network with specialized architecture for trading data.
    """
    def __init__(self, *args, **kwargs):
        super(PippoActorCriticPolicy, self).__init__(
            *args,
            **kwargs,
            net_arch=dict(pi=[128, 128], qf=[128, 128])
        )
