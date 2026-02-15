/**
 * @author myself <myself@example.com>
 * @date 2026-02-15 21:45:04.689620853 UTC
 */

export interface DashboardProps{
   users: number;
   revenue: number;
   orders: number;

}

export function Dashboard(props: DashboardProps){
    return (
        <div className="dashboard">
            <h2>Dashboard Component</h2>
           <p>users: {props.users}</p>
           <p>revenue: {props.revenue}</p>
           <p>orders: {props.orders}</p>

        </div>
    );
}
