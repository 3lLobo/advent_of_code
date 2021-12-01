import numpy as np


def main():
    """
    Solution advent of code 21 - day 1
    """
    data = np.loadtxt('2021/day1/input.txt', dtype=int)
    
    data_aug = data[1:] - data[:-1]
    n_increased = np.sum(data_aug > 0)

    print('Following datapoints increased in {} of total {} cases.'.format(n_increased, len(data)))
    
    # Part 2

    n_over = len(data) % 3
    n_window = 3
    m_shape = (-1, n_window)

    m = np.array([np.sum(data[:-n_over].reshape(m_shape), axis=1),
                    np.sum(data[1:1-n_over].reshape(m_shape), axis=1),
                    np.sum(data[2:].reshape(m_shape), axis=1)])

    m_increased = 0
    for dim in range(n_window):
        dim2 = (dim+1) % n_window
        if dim == 2:
            m_aug = m[dim2,1:] - m[dim,:-1]
        else:
            m_aug = m[dim2,:] - m[dim,:]
        m_increased += np.sum(m_aug > 0)

    print('Slinding window of 3 datapoints increased in {} of total {} cases.'.format(m_increased, m.shape[1]))
    

if __name__=='__main__':
    main()
