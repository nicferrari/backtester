use std::collections::HashMap;
use csv::Writer;
use std::error::Error;
use crate::strategies::Strategy;
use crate::{orders};
use crate::datas::Data;
use crate::orders::Order;

///to create a Backtest use Backtest::new()
#[derive(Clone)]
pub struct Backtest{
    quotes:Data,
    strategy:Strategy,
    position:Vec<f64>,
    account:Vec<f64>,
    commission: Commission,
}

#[derive(Clone)]
pub struct Commission{
    rate:f64,
    floor:f64,//not implemented
}

impl Default for Commission{
    fn default() -> Self {
        Self{rate:0.1,
        floor:0.,//not implemented
        }
    }

}

#[derive(PartialEq)]
enum Stance{
    LONG,
    NULL,
    SHORT,
}

impl Backtest{
    pub fn new(quotes:Data,strategy: Strategy,account:f64)->Self{
        //pub fn new(quotes:Data,strategy: Strategy,account:f64)->Result<Self>{
        let length = quotes.timestamps().len();
        let position = vec![0.;length];
        let account = vec![account;length];
        let mut _backtest = Backtest{
            quotes:quotes,
            strategy:strategy,
            position:position,
            account:account,
            commission:Commission::default(),
        };
        _backtest.calculate();
        _backtest
    }
    pub fn quotes(&self)->&Data{return &self.quotes}
    pub fn orders(&self)->Vec<Order>{return self.strategy.choices()}
    pub fn position(&self)->Vec<f64>{return self.position.clone()}
    pub fn account(&self)->Vec<f64>{return self.account.clone();}
    pub fn strategy(&self)->Strategy{return self.strategy.clone();}
    ///function which display the requested log values of the calculations made period by period
    /// available choices at the moment are: close, open, low, high, position, account, indicator(s, up to 2)
    pub fn log(&self, list:&[&str]){
        let mut data_functions: HashMap<&str, fn(&Data)->Vec<f64>>=HashMap::new();
        data_functions.insert("close", Data::close);
        data_functions.insert("open", Data::open);
        data_functions.insert("low",Data::low);
        data_functions.insert("high",Data::high);
        let mut backtest_functions: HashMap<&str, fn(&Backtest)->Vec<f64>>=HashMap::new();
        backtest_functions.insert("position",Backtest::position);
        backtest_functions.insert("account",Backtest::account);
        let mut strategy_function: HashMap<&str, fn(&Strategy)->Option<Vec<Vec<f64>>>>=HashMap::new();
        strategy_function.insert("indicator",Strategy::indicator);
        for i in 0..self.quotes.timestamps().len(){
            print!("Date = {:} - ",&self.quotes.timestamps()[i].format("%Y-%m-%d"));
            for j in list{
                if let Some(func) = data_functions.get(j){
                    let value = func(&self.quotes)[i];
                    print!("{} = {:.2}  ",j,value)
                };
                if let Some(func) = backtest_functions.get(j){
                    let value = func(&self)[i];
                    print!("{} = {:.2}  ",j,value)
                };
                if let Some(func) = strategy_function.get(j){
                    let value = func(&self.strategy);
                    if let Some(first_vec) = value.iter().flatten().next(){print!("{} = {:.2}  ",j,first_vec[i])}
                    if let Some(second_vec) = value.iter().flatten().skip(1).next(){print!("{} = {:.2}  ",j,second_vec[i])}
                    // to do: extend to n-case
                }
            }
            print!("   - net worth = {:.2}",self.quotes.close()[i]*self.position()[i]+self.account()[i]);
            println!();
        }
    }
    ///function which does the actual backtest and returns a vector of (signed) positions and account values
    fn calculate(&mut self){
        let mut stance = Stance::NULL;
        let mut previous_position = 0.;
        let mut previous_account = 100000.;
        for i in 1..self.quotes.timestamps().len(){
            match self.strategy.choices()[i-1]{
                orders::Order::BUY=>{
                    if stance!=Stance::LONG{
                        let networth = previous_account + previous_position * self.quotes.open()[i]*(1.-previous_position.signum()*self.commission.rate);
                        //self.position[i] = ((self.account[i]/self.quotes.close()[i]) as i64) as f64;
                        self.position[i] = ((networth/(self.quotes.open()[i]*(1.+self.commission.rate))) as i64) as f64;
                        self.account[i] = networth-self.position[i]*(self.quotes.open()[i]*(1.+self.commission.rate));
                        stance = Stance::LONG;
                    } else {
                        self.position[i] = previous_position;
                        self.account[i] = previous_account;
                    }
                }
                orders::Order::SHORTSELL=>{
                    if stance!=Stance::SHORT{
                        let networth = previous_account + previous_position * self.quotes.open()[i]*(1.-previous_position.signum()*self.commission.rate);
                        //self.position[i] = -((self.account[i]/self.quotes.close()[i]) as i64) as f64;
                        self.position[i] = -((networth/self.quotes.open()[i]*(1.-self.commission.rate)) as i64) as f64;
                        self.account[i] = networth-self.position[i]*self.quotes.open()[i]*(1.-self.commission.rate);
                        stance = Stance::SHORT;
                    } else {
                        self.position[i] = previous_position;
                        self.account[i] = previous_account;
                    }
                }
                orders::Order::NULL=>{
                    if stance!=Stance::NULL{
                        let networth = previous_account + previous_position * self.quotes.open()[i]*(1.-previous_position.signum()*self.commission.rate);
                        self.position[i]=0.;
                        self.account[i]=networth;
                        stance = Stance::NULL;
                    } else {
                        self.position[i] = previous_position;
                        self.account[i] = previous_account;
                    }
                }
            }
            previous_account = self.account[i];
            previous_position = self.position[i];
        }
    }
    ///print Backtest to csv.
    ///Indicator can only be 1 or 2 at the moment
    pub fn to_csv(&self, filename:&str)->Result<(), Box<dyn Error>>{
        let mut wrt = Writer::from_path(filename)?;
        //the part below can be macro-ed
        let timestamps_t:Vec<Vec<String>> = self.quotes.timestamps().iter().map(|e|vec![e.to_string()[0..10].to_string()]).collect();
        let open_t:Vec<Vec<String>> = self.quotes.open().iter().map(|e|vec![e.to_string()]).collect();
        let close_t:Vec<Vec<String>> = self.quotes.close().iter().map(|e|vec![e.to_string()]).collect();
        let choices_t:Vec<Vec<String>> = self.strategy.choices().iter().map(|e|vec![e.to_string().to_string()]).collect();

        let position_t:Vec<Vec<String>> = self.position.iter().map(|e|vec![e.to_string()]).collect();
        let account_t:Vec<Vec<String>> = self.account.iter().map(|e|vec![e.to_string()]).collect();

        let mktvalue_t:Vec<Vec<String>> = self.position.iter().zip(self.quotes().close().iter()).map(|(a,b)|a*b).collect::<Vec<_>>().iter().map(|e|vec![e.to_string()]).collect();
        let networth_t:Vec<Vec<String>> = (self.position.iter().zip(self.quotes().close().iter())).zip(self.account.iter()).map(|((a,b),c)|a*b+c).collect::<Vec<_>>().iter().map(|e|vec![e.to_string()]).collect();

//        let flows_t:Vec<String> = std::iter::once(None).chain(self.position.windows(2).map(|window|Some(window[1]-window[0]))).map(|e|match e { Some(value)=>value.to_string(),
//        None=>"None".to_string()}).collect();
/*        let flows_t:Vec<String> = std::iter::once(None).chain(self.position.windows(2).zip(self.quotes.open().windows(2))
            .map(|(window,window1)|Some(-(window[1]-window[0])*window1[1]))).map(|e|match e { Some(value)=>value.to_string(),
            None=>"None".to_string()}).collect();

        let commission_t:Vec<String> = std::iter::once(None).chain(flows_t.windows(2).zip(networth_t.windows(2))
                                                          .map(|(window,window1)| window[1].parse::<f64>().unwrap())).map(|e|match e {
            Some(value)=>value.to_string(), None=>"None".to_string()}).collect();
*/
        let flows:Vec<f64> = std::iter::once(None).chain(self.position.windows(2).zip(self.quotes.open().windows(2))
                    .map(|(window,window1)|Some(-(window[1]-window[0])*window1[1]))).map(|e|match e{
            Some(value)=>value,None=>0.}).collect();
        //let flows_t:Vec<String> = flows.iter().map(|e|match e {Some(value)=>value.to_string(),None=>"None".to_string()}).collect();
        let flows_t:Vec<String> = flows.iter().map(|e|e.to_string()).collect();

        let commission_t:Vec<String> = std::iter::once(None).chain(self.account.windows(2).zip(flows.windows(2)).map(
            |(window,window1)|Some(window[0]-window[1]+window1[1]))).map(|e|match e{Some(value)=>(-1.*value).to_string(),None=>0.to_string()}).collect();

        //        let indicator1_t:Vec<Vec<String>> = self.strategy.indicator().iter().next().unwrap().iter().next().unwrap().iter().map(|e|vec![e.to_string()]).collect();
        //        let indicator2_t:Vec<Vec<String>> = self.strategy.indicator().iter().skip(1).next().unwrap().iter().next().unwrap().iter().map(|e|vec![e.to_string()]).collect();
        //the below work but indicator2 is Option, so could break if only one, besides also need to manage n-case
                let indicator1_t:Vec<Vec<String>> = self.strategy.indicator().iter().flatten().next().unwrap().iter().map(|e|vec![e.to_string()]).collect();
        //        let indicator2_t:Vec<Vec<String>> = self.strategy.indicator().iter().flatten().skip(1).next().unwrap().iter().map(|e|vec![e.to_string()]).collect();

                let indicator2_t:Option<Vec<Vec<String>>>;
                match self.strategy.indicator(){
                    Some(innervec)=>{
                        if innervec.len() >=2{
                            indicator2_t = Some(self.strategy.indicator().iter().flatten().skip(1).next().unwrap().iter().map(|e|vec![e.to_string()]).collect());
                        } else {indicator2_t = None}
                    }
                    None=>{
                        indicator2_t = None;
                    }
                }

        /*
                wrt.serialize(("DATE","CLOSE","CHOICES","INDIC1","INDIC2"))?;
                for ((((col1,col2),col3),col4),col5) in timestamps_t.iter().zip(close_t.iter()).zip(choices_t.iter()).zip(indicator1_t.iter()).zip(Some(indicator2_t).iter()){
                    wrt.serialize((col1,col2,col3,col4,col5))?;
                }*/
                if let Some(ind2_t) = indicator2_t{
                    wrt.serialize(("DATE","OPEN","CLOSE","CHOICES","INDIC1","INDIC2","ACCOUNT","POSITION","MKTVALUE","NETWORTH","FLOW","COMMISSION"))?;
                    for (((((((((((col1,col2),col3),col4),col5),col6),col7),col8),col9),col10),col11),col12) in timestamps_t.iter().zip(open_t.iter()).zip(close_t.iter()).zip(choices_t.iter())
                        .zip(indicator1_t.iter()).zip(ind2_t.iter()).zip(account_t.iter()).zip(position_t.iter()).zip(mktvalue_t.iter()).zip(networth_t.iter()).zip(flows_t.iter()).zip(commission_t.iter()){
                        wrt.serialize((col1,col2,col3,col4,col5,col6,col7,col8,col9,col10,col11,col12))?;
                    } }
                else
                {
                    wrt.serialize(("DATE","OPEN","CLOSE","CHOICES","INDIC1","ACCOUNT","POSITION","MKTVALUE","NETWORTH","FLOW","COMMISSION"))?;
                    for ((((((((((col1,col2),col3),col4),col5),col6),col7),col8),col9),col10),col11) in timestamps_t.iter().zip(open_t.iter()).zip(close_t.iter()).zip(choices_t.iter())
                        .zip(indicator1_t.iter()).zip(account_t.iter()).zip(position_t.iter()).zip(mktvalue_t.iter()).zip(networth_t.iter()).zip(flows_t.iter()).zip(commission_t.iter()){
                        wrt.serialize((col1,col2,col3,col4,col5,col6,col7,col8,col9,col10,col11))?;
                    }
                }
                wrt.flush()?;
                Ok(())
            }
        }