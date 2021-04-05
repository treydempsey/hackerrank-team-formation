#!/bin/ruby

require 'json'
require 'stringio'



#
# Complete the 'teamFormation' function below.
#
# The function is expected to return a LONG_INTEGER.
# The function accepts following parameters:
#  1. INTEGER_ARRAY score
#  2. INTEGER team_size
#  3. INTEGER k
#


def teamFormation(score, team_size, k)
    team_scores = []
    while team_scores.size < team_size
        # If k is smaller than the total number of scores
        if k < score.size
            # Get the first k elements from scores
            front = score[0..(k-1)]
            
            # Get the last k elements from scores
            back = score[-k..]
        
            # Find the maximum value of both slices
            front_max = front.max()
            back_max = back.max()
            
            # Of the front and back slices prefer the maximum from the front slice
            if front_max >= back_max
                # Save the selected value for return
                team_scores.push(front_max)
                # Determine the index of the value from the front slice
                front_index = front.index(front_max)
                # Remove the value from the scores array
                score.delete_at(front_index)
            else
                # Save the selected value for return
                team_scores.push(back_max)
                # Determine the index of the value from the back slice
                back_index = back.index(back_max) + score.size - k
                # Remove the value from the scores array
                score.delete_at(back_index)
            end
        else
            # Find the maximum value and save it for return
            selected = score.max()
            team_scores.push(selected)
            # Find the index of the first occurance of selected
            index = score.index(selected)
            # Remove the value from the scores array
            score.delete_at(index)
        end
    end
    
    team_scores.sum()
end

fptr = File.open(ENV['OUTPUT_PATH'], 'w')

score_count = gets.strip.to_i

score = Array.new(score_count)

score_count.times do |i|
    score_item = gets.strip.to_i
    score[i] = score_item
end

team_size = gets.strip.to_i

k = gets.strip.to_i

result = teamFormation score, team_size, k

fptr.write result
fptr.write "\n"

fptr.close()
