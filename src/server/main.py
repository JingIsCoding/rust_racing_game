import os
import neat
import flask
from flask import Flask, request, jsonify, make_response
from flask_cors import CORS

app = Flask(__name__)
CORS(app)
local_dir = os.path.dirname(__file__)
config_file = os.path.join(local_dir, 'config-feedforward')
config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                     neat.DefaultSpeciesSet, neat.DefaultStagnation,
                     config_file)
p = neat.Population(config)

@app.route('/api/gen_network', methods=['POST'])
def gen_network():
    json_request = request.json
    num_of_cars = json_request["num_of_cars"]
    num_of_args = json_request["num_of_args"]
    num_of_outputs = json_request["num_of_outputs"]

    config.pop_size = num_of_cars
    config.num_of_input = num_of_args
    config.num_outputs = num_of_outputs

    p = neat.Population(config)
    print(p.population.values())
    return jsonify({})

@app.route('/api/evaluate_network', methods=['POST'])
def evalute():
    data = request.json
    inputs = data["inputs"]
    genomes = p.population.values()
    outputs = []
    if len(inputs) > len(genomes):
        response = make_response('not enough genomes', 422)  # 204 indicates No Content
        return response

    for index, genome in enumerate(genomes):
        if index < len(inputs):
            net = neat.nn.FeedForwardNetwork.create(genome, p.config)
            output = net.activate(inputs[index])
            outputs.append(output)

    return jsonify(outputs)

@app.route('/api/set_fitness', methods=['POST'])
def set_fitnesses():
    json = request.json
    fitnesses = json['fitnesses']
    genomes = p.population.values()
    if len(fitnesses) < len(genomes):
        response = make_response('', 422)  # 204 indicates No Content
        return response
    for index, genome in enumerate(genomes):
        genome.fitness = fitnesses[index]
    # Create an empty response with a specific status code
    response = make_response('', 204)  # 204 indicates No Content
    return response


@app.route('/api/next_gen', methods=['POST'])
def next_gen():
    for genome in p.population.values():
        print(genome.fitness)
    for sid, s in p.species.species.items():
        print(s.get_fitnesses())

    p.population = p.reproduction.reproduce(p.config, p.species, p.config.pop_size, p.generation)
    p.species.speciate(p.config, p.population, p.generation)
    p.generation += 1
    return jsonify({})


if __name__ == '__main__':
    app.run(host= '0.0.0.0',port='3030',debug=True)
