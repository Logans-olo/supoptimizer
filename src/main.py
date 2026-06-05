from pyscipopt import Model

def main():
    scip = Model()
    x = scip.addVar(vtype='C', lb=0, ub=None, name='x')
    y = scip.addVar(vtype='C', lb=0, ub=None, name='y')
    z = scip.addVar(vtype='C', lb=0, ub=None, name='z')
    cons_1 = scip.addCons(x + y <= 5, name="cons_1")
    cons_2 = scip.addCons(x + z >= 3, name="cons_2")
    cons_3 = scip.addCons(y + z == 4, name="cons_3")
    scip.setObjective(2 * x + 3 * y - 5 * z, sense="minimize")
    scip.optimize()

    solve_time = scip.getSolvingTime()
    num_nodes = scip.getNTotalNodes() 
# Note that getNNodes() is only the number of nodes for the current run (resets at restart)
    obj_val = scip.getObjVal()
    for scip_var in [x, y, z]:
        print(f"Variable {scip_var.name} has value {scip.getVal(scip_var)}")


if __name__ == "__main__":
    main()
