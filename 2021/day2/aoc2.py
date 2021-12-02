import pandas as pd
from typing import List, Dict


class submarine(object):
    """Yellow submarine diving through the ocean of data

    Args:
        object ([type]): [description]
    """
    def __init__(self, init_pos:List = [0,0], color:str = 'Yellow') -> None:
        super().__init__()
        self.h_pos = init_pos[0]
        self.v_pos = init_pos[1]
        self.color = color
        self.welcome_msg = f'Welcome onboard the {self.color} submarine!'
        print(self.welcome_msg)

        self.engine = {'forward': {'move': 'horizontal', 'dir': 1},
                        'up': {'move': 'vertical', 'dir': -1},
                        'down': {'move': 'vertical', 'dir': 1}
                        }

    def navigate(self, instructions: pd.DataFrame) -> Dict:
        """Take instructions from the capitain and navigate.

        Args:
            instructions (pd.DataFrame): Instructions to navigate the submarine

        Returns:
            Dict: Final position
        """
        commands = instructions.Command.unique()

        for com in commands:
            move = instructions.loc[instructions.Command == com].Value.sum()
            dir = self.engine[com]['move']

            if dir == 'horizontal':
                self.h_pos += move*self.engine[com]['dir']
            elif dir == 'vertical':
                self.v_pos += move*self.engine[com]['dir']
            else:
                print('Lost in space!')
                raise KeyError
                    
            print(f'Captain, we went {move} yards {dir}')
        
        return {'horizontal': self.h_pos, 'vertical': self.v_pos}
    
    @staticmethod
    def load_data(data_path:str) -> pd.DataFrame:
        """Load the data and return it as DF.

        Args:
            data_path (str): Path to the data

        Returns:
            pd.DataFrame: dataframe with data
        """
        df = pd.read_csv(data_path, sep=' ', header=None)
        df.columns = ['Command', 'Value']
        assert len(df.columns) == 2

        return df


if __name__=="__main__":
    """Run script and get the treasure!
    Ayeee
    """
    data = submarine.load_data('2021/day2/input.txt')

    my_sub = submarine()

    location = my_sub.navigate(data)

    product = 1
    for key, val in location.items():
        product = product * val
        print(key, ':\t', val)
    
    print(f'Answer day 2: {product}')
